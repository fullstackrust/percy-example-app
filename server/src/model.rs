use crate::schema::{InputJob, OutputJob};
use bytevec::errors::ByteVecError;
use bytevec::{ByteDecodable, ByteEncodable};
use chrono::Utc;
use dirs;
use sled::{Db, IVec};
use ulid::Ulid;

// Byte serialization models
bytevec_decl! {
    #[derive(PartialEq, Eq, Debug)]
    pub struct Job {
        pub id: String,         // TODO type
        pub date_added: String, // TODO type
        pub job_desc: String,
        pub job_name: String,
        pub job_rate_type: String, // TODO type
        pub job_rate: String,      // TODO type
        pub user_id: String,       // TODO user_id
        pub user_name: String
    }
}

impl Into<OutputJob> for Job {
    fn into(self) -> OutputJob {
        OutputJob {
            id: self.id,
            date_added: self.date_added,
            job_desc: self.job_desc,
            job_name: self.job_name,
            job_rate_type: self.job_rate_type,
            job_rate: self.job_rate,
            user_id: self.user_id,
            user_name: self.user_name,
        }
    }
}

// Low-level Database Helpers
pub struct Database {
    db: Db,
}

impl Database {
    pub fn new() -> Result<Self, ()> {
        let path = Path::new("./data");

        match Db::start_default(path) {
            Ok(db) => Result::Ok(Database { db }),
            Err(err) => {
                error!("Error creating database: {:?}", err);
                Result::Err(())
            }
        }
    }

    pub fn set(&self, key: String, value: Vec<u8>) -> Result<(), sled::Error> {
        match self.db.set(key, value) {
            Ok(_old_val) => Result::Ok(()),
            Err(err) => {
                error!("Error inserting key into database: {}", err);
                Result::Err(err)
            }
        }
    }

    pub fn get(&self, key: String) -> Result<IVec, sled::Error> {
        match self.db.get(key) {
            Ok(val) => Result::Ok(val.unwrap()),
            Err(err) => {
                error!("Error inserting key into database: {}", err);
                Result::Err(err)
            }
        }
    }

    pub fn get_gt(&self, key: String) -> Result<Vec<IVec>, sled::Error> {
        let mut results: Vec<IVec> = vec![];
        let mut finished = false;
        let mut next_key = key.into_bytes();

        while !finished {
            let result = self.db.get_gt(&next_key).unwrap();

            if !result.is_none() {
                let (key, value) = result.unwrap();
                results.push(value);
                next_key = key;
            } else {
                finished = true;
            }
        }

        Result::Ok(results)
    }

    // Model-specific serializers and deserializers
    pub fn set_job(&self, job: &InputJob) -> Result<OutputJob, ()> {
        let id = Ulid::new().to_string();
        let date_added = Utc::now().to_string();
        let user_id = Ulid::new().to_string(); // TODO temp

        let job: Job = Job {
            id,
            date_added,
            job_desc: job.desc.clone(),
            job_name: job.name.clone(),
            job_rate_type: "hourly".to_string(),
            job_rate: job.rate.clone(),
            user_id,                     // TODO use user id
            user_name: job.user.clone(), // TODO lookup user id
        };

        let key = format!("job:{}", job.id);
        let value = job.encode::<u8>().unwrap();

        match self.set(key, value) {
            Ok(_old_val) => {
                info!("Job inserted: {:?}", job);
                Result::Ok(job.into())
            }
            Err(err) => {
                error!("Error inserting key into database: {}", err);
                Result::Err(())
            }
        }
    }

    pub fn get_job(&self, job: Job) -> Result<Job, ByteVecError> {
        let key = format!("job:{}", job.id);
        let value = self.get(key).unwrap();
        Job::decode::<u8>(&value)
    }

    pub fn get_jobs(&self) -> Result<Vec<OutputJob>, ByteVecError> {
        let key = format!("job:");
        self.get_gt(key)
            .unwrap()
            .iter()
            .map(|value: &IVec| {
                let job = Job::decode::<u8>(&value).unwrap();
                Result::Ok(job.into())
            })
            .collect()
    }
}
