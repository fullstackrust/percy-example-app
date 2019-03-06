use crate::schema::{GraphQLJob, InputJob};
use bytevec::errors::ByteVecError;
use bytevec::{ByteDecodable, ByteEncodable};
use dirs;
use sled::{Db, IVec};

// Byte serialization models
bytevec_decl! {
    #[derive(PartialEq, Eq, Debug)]
    pub struct Job {
        name: String,
        desc: String,
        user: String,
        rate: String
    }
}

impl From<&InputJob> for Job {
    fn from(job: &InputJob) -> Self {
        let new_job = job.clone();
        Job {
            name: new_job.name.clone(),
            desc: new_job.desc.clone(),
            user: new_job.user.clone(),
            rate: new_job.rate.clone(),
        }
    }
}

impl From<&InputJob> for GraphQLJob {
    fn from(job: &InputJob) -> Self {
        let new_job = job.clone();
        GraphQLJob {
            name: new_job.name.clone(),
            desc: new_job.desc.clone(),
            user: new_job.user.clone(),
            rate: new_job.rate.clone(),
        }
    }
}

// Low-level Database Helpers
pub struct Database {
    db: Db,
}

impl Database {
    pub fn new() -> Result<Self, ()> {
        let mut path = dirs::home_dir().unwrap();
        path.push(".data");
        path.push("innotrade_contractor");

        match Db::start_default(path) {
            Ok(db) => Result::Ok(Database { db }),
            Err(err) => {
                println!("Error creating database: {:?}", err);
                Result::Err(())
            }
        }
    }

    pub fn set(&self, key: String, value: Vec<u8>) -> Result<(), sled::Error<()>> {
        match self.db.set(key, value) {
            Ok(_old_val) => Result::Ok(()),
            Err(err) => {
                println!("Error inserting key into database: {}", err);
                Result::Err(err)
            }
        }
    }

    pub fn get(&self, key: String) -> Result<IVec, sled::Error<()>> {
        match self.db.get(key) {
            Ok(val) => Result::Ok(val.unwrap()),
            Err(err) => {
                println!("Error inserting key into database: {}", err);
                Result::Err(err)
            }
        }
    }

    pub fn get_gt(&self, key: String) -> Result<Vec<IVec>, sled::Error<()>> {
        let mut results: Vec<IVec> = vec![];

        while let Ok(result) = self.db.get_gt(&key) {
            results.push(result.unwrap().1);
        }

        Result::Ok(results)
    }

    // // Model-specific serializers and deserializers
    pub fn set_job(&self, job: &InputJob) -> Result<GraphQLJob, ()> {
        // let job: Job = Job {
        //     name: job.name,
        //     desc: job.desc,
        //     user: job.user,
        //     rate: job.rate,
        // };

        let key = format!("job:{}", job.name);
        let value = Job::from(job.clone()).encode::<u8>().unwrap();

        match self.set(key, value) {
            Ok(val) => Result::Ok(GraphQLJob::from(job.clone())),
            Err(err) => {
                println!("Error inserting key into database: {}", err);
                Result::Err(())
            }
        }
    }

    pub fn get_job(&self, job: Job) -> Result<Job, ByteVecError> {
        let key = format!("job:{}", job.name);
        let value = self.get(key).unwrap();
        Job::decode::<u8>(&value)
    }

    pub fn get_jobs(&self) -> Result<Vec<GraphQLJob>, ByteVecError> {
        let key = format!("job:");
        self.get_gt(key)
            .unwrap()
            .iter()
            .map(|value: &IVec| {
                let job = Job::decode::<u8>(&value).unwrap();
                Result::Ok(GraphQLJob {
                    name: job.name,
                    desc: job.desc,
                    user: job.user,
                    rate: job.rate,
                })
            })
            .collect()
    }
}
