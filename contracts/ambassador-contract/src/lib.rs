#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, contracterror,
    Env, Address,
    BytesN, // Used for the Hash
    Symbol, symbol_short, // Used for Events
    Vec,    // Used for batch operations
    String, // Used for nicknames
};

// --- Custom Error Definitions ---
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    NoActiveSession = 3,
    IncorrectHash = 4,
    AlreadyRegistered = 5,
    InvalidNickname = 6,
}

// --- User Profile Struct ---
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserProfile {
    pub nickname: String,
    pub registered_at: u32, // Ledger sequence number when profile was created/updated
}

// --- Storage Key Definitions ---
// RECOMMENDATION 5: Added Debug, Eq, PartialEq
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StorageKey {
    Admin,
    ActiveHash,
    Presence(BytesN<32>, Address),
    UserProfile(Address),
}

// --- Contract Definition ---
#[contract]
pub struct AttendanceContract;

// --- Contract Implementation ---
#[contractimpl]
impl AttendanceContract {

    // --- POINT 3: New TTL pattern ---
    // Threshold: ~7 days (17280 ledgers/day * 7 days)
    const TTL_THRESHOLD: u32 = 120_960;

    // Bump for instance data and sessions: ~30 days
    const TTL_BUMP_30D: u32 = 518_400;

    // Bump for user profiles: ~90 days
    const TTL_BUMP_90D: u32 = 1_555_200;

    /// Initializes the contract, setting the administrator.
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&StorageKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }

        admin.require_auth();
        env.storage().instance().set(&StorageKey::Admin, &admin);

        // --- CRITICAL 1: Add TTL extension for instance storage ---
        env.storage().instance().extend_ttl(Self::TTL_THRESHOLD, Self::TTL_BUMP_30D);

        // --- RECOMMENDATION 6: Add initialization event ---
        env.events().publish(
            (symbol_short!("init"),),
                             admin
        );

        Ok(())
    }

    /// (Admin only) Sets the active attendance hash, starting a new session.
    pub fn set_hash(env: Env, new_hash: BytesN<32>) -> Result<(), Error> {
        let admin: Address = env.storage().instance()
        .get(&StorageKey::Admin)
        .ok_or(Error::NotInitialized)?;

        admin.require_auth();

        env.storage().persistent().set(&StorageKey::ActiveHash, &new_hash);

        // --- POINT 3: Use the new TTL pattern ---
        env.storage().persistent().extend_ttl(
            &StorageKey::ActiveHash,
            Self::TTL_THRESHOLD,
            Self::TTL_BUMP_30D
        );

        env.events().publish(
            (symbol_short!("new_sess"),),
                             new_hash
        );

        Ok(())
    }

    /// (User function) Registers the caller's presence for the active session.
    pub fn register(env: Env, user: Address, submitted_hash: BytesN<32>) -> Result<(), Error> {
        user.require_auth();

        let stored_hash: BytesN<32> = env
        .storage()
        .persistent()
        .get(&StorageKey::ActiveHash)
        .ok_or(Error::NoActiveSession)?;

        // --- POINT 3: Use the new TTL pattern ---
        env.storage().persistent().extend_ttl(
            &StorageKey::ActiveHash,
            Self::TTL_THRESHOLD,
            Self::TTL_BUMP_30D
        );

        if submitted_hash != stored_hash {
            return Err(Error::IncorrectHash);
        }

        let presence_key = StorageKey::Presence(stored_hash.clone(), user.clone());

        if env.storage().persistent().has(&presence_key) {
            return Err(Error::AlreadyRegistered);
        }

        env.storage().persistent().set(&presence_key, &true);

        // --- POINT 3: Use the new TTL pattern ---
        env.storage().persistent().extend_ttl(
            &presence_key,
            Self::TTL_THRESHOLD,
            Self::TTL_BUMP_30D
        );

        let profile_key = StorageKey::UserProfile(user.clone());
        let nickname = if let Some(profile) = env.storage().persistent().get::<StorageKey, UserProfile>(&profile_key) {
            // Extends the profile's TTL upon registration
            env.storage().persistent().extend_ttl(
                &profile_key,
                Self::TTL_THRESHOLD,
                Self::TTL_BUMP_90D
            );
            profile.nickname
        } else {
            String::new(&env)
        };

        // --- RECOMMENDATION 9: Remove unnecessary clones ---
        env.events().publish(
            (symbol_short!("present"),),
                             (user, stored_hash, nickname) // (user, session_hash, nickname)
        );

        Ok(())
    }

    /// (Admin only) Transfers admin rights to a new address.
    pub fn transfer_admin(env: Env, new_admin: Address) -> Result<(), Error> {
        let current_admin: Address = env.storage().instance()
        .get(&StorageKey::Admin)
        .ok_or(Error::NotInitialized)?;

        current_admin.require_auth();
        new_admin.require_auth();

        env.storage().instance().set(&StorageKey::Admin, &new_admin);

        // --- CRITICAL 2: Add TTL extension for instance storage ---
        env.storage().instance().extend_ttl(Self::TTL_THRESHOLD, Self::TTL_BUMP_30D);

        // --- RECOMMENDATION 9: Remove unnecessary clones ---
        env.events().publish(
            (symbol_short!("adm_xfer"),),
                             (current_admin, new_admin)
        );

        Ok(())
    }

    /// (User function) Creates or updates a user's profile with a nickname.
    pub fn set_profile(env: Env, user: Address, nickname: String) -> Result<(), Error> {
        user.require_auth();

        if nickname.len() < 3 || nickname.len() > 32 {
            return Err(Error::InvalidNickname);
        }

        let profile = UserProfile {
            nickname: nickname.clone(),
            registered_at: env.ledger().sequence(),
        };

        let profile_key = StorageKey::UserProfile(user.clone());
        env.storage().persistent().set(&profile_key, &profile);

        // --- POINT 3: Use the new TTL pattern (with 90 days) ---
        env.storage().persistent().extend_ttl(
            &profile_key,
            Self::TTL_THRESHOLD,
            Self::TTL_BUMP_90D
        );

        // --- RECOMMENDATION 9: Remove unnecessary clones ---
        env.events().publish(
            (symbol_short!("profile"),),
                             (user, nickname)
        );

        Ok(())
    }

    /// (View function) Retrieves a user's profile (if it exists).
    pub fn get_profile(env: Env, user: Address) -> Option<UserProfile> {
        let profile_key = StorageKey::UserProfile(user);

        if let Some(profile) = env.storage().persistent().get::<StorageKey, UserProfile>(&profile_key) {
            // --- POINT 3: Use the new TTL pattern (with 90 days) ---
            env.storage().persistent().extend_ttl(
                &profile_key,
                Self::TTL_THRESHOLD,
                Self::TTL_BUMP_90D
            );
            Some(profile)
        } else {
            None
        }
    }


    /// (View function) Checks if a user is registered for the CURRENT active session.
    pub fn check_presence(env: Env, user: Address) -> bool {

        let current_hash: BytesN<32> = match env.storage().persistent().get(&StorageKey::ActiveHash) {
            Some(hash) => hash,
            None => return false,
        };

        // --- POINT 3: Use the new TTL pattern ---
        env.storage().persistent().extend_ttl(
            &StorageKey::ActiveHash,
            Self::TTL_THRESHOLD,
            Self::TTL_BUMP_30D
        );

        let presence_key = StorageKey::Presence(current_hash, user);

        let is_present = env.storage().persistent().get(&presence_key).unwrap_or(false);

        if is_present {
            // --- POINT 3: Use the new TTL pattern ---
            env.storage().persistent().extend_ttl(
                &presence_key,
                Self::TTL_THRESHOLD,
                Self::TTL_BUMP_30D
            );
        }

        is_present
    }

    /// (View function) Returns the current admin address.
    pub fn get_admin(env: Env) -> Result<Address, Error> {
        // --- CRITICAL 4: Add TTL extension on read ---
        env.storage().instance().extend_ttl(Self::TTL_THRESHOLD, Self::TTL_BUMP_30D);

        env.storage().instance()
        .get(&StorageKey::Admin)
        .ok_or(Error::NotInitialized)
    }

    /// (View function) Returns the current active session hash (if any).
    pub fn get_session(env: Env) -> Option<BytesN<32>> {
        if let Some(hash) = env.storage().persistent().get(&StorageKey::ActiveHash) {
            // --- POINT 3: Use the new TTL pattern ---
            env.storage().persistent().extend_ttl(
                &StorageKey::ActiveHash,
                Self::TTL_THRESHOLD,
                Self::TTL_BUMP_30D
            );
            Some(hash)
        } else {
            None
        }
    }

    /// (View function) Check presence for multiple users at once.
    pub fn check_batch(env: Env, users: Vec<Address>) -> Vec<bool> {
        let current_hash: BytesN<32> = match Self::get_session(env.clone()) {
            Some(hash) => hash,
            None => return Vec::new(&env),
        };

        let mut results = Vec::new(&env);
        for user in users.iter() {
            let presence_key = StorageKey::Presence(current_hash.clone(), user);

            let is_present = env.storage().persistent().get(&presence_key).unwrap_or(false);

            if is_present {
                // --- POINT 3: Use the new TTL pattern ---
                env.storage().persistent().extend_ttl(
                    &presence_key,
                    Self::TTL_THRESHOLD,
                    Self::TTL_BUMP_30D
                );
            }
            results.push_back(is_present);
        }
        results
    }
}
