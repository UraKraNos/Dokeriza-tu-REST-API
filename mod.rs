// External imports
use bson::{doc, Document};
use mongodb::results::{DeleteResult, UpdateResult, InsertOneResult};
use mongodb::{error::Error, Collection};
use serde::{Deserialize, Serialize};
// External constructors
extern crate serde;
extern crate serde_json;

// Estructure data for DB
#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub carrera: String,
    pub materias_1_s: String,
    pub materias_2_s: String,
    pub materias_3_s: String,
    pub materias_4_s: String,
    pub materias_5_s: String,
    pub materias_6_s: String,
    pub materias_7_s: String,
    pub materias_8_s: String,
    pub materias_9_s: String,
    pub especialidad: String,
    pub materias_e: String,
}

// Reference colection clone
#[derive(Clone)]
pub struct ApiService {
    collection: Collection,
}

// Transform data to mongo db document
fn data_to_document(data: &Data) -> Document {
    let Data {
        carrera,
        materias_1_s,
        materias_2_s,
        materias_3_s,
        materias_4_s,
        materias_5_s,
        materias_6_s,
        materias_7_s,
        materias_8_s,
        materias_9_s,
        especialidad,
        materias_e,
    } = data;
    doc! {
        "carrera": carerra,
        "materias_1_s": materias_1_s,
        "materias_2_s": materias_2_s,
        "materias_3_s": materias_3_s,
        "materias_4_s": materias_4_s,
        "materias_5_s": materias_5_s,
        "materias_6_s": materias_6_s,
        "materias_7_s": materias_7_s,
        "materias_8_s": materias_8_s,
        "materias_9_s": materias_9_s,
        "especialidad": especialidad,
        "materias_e": materias_e,
    }
}

// Functions with quieries to Mongo
impl ApiService {
    pub fn new(collection: Collection) -> ApiService {
        ApiService { collection }
    }

    // Insert data to Mongo DB
    pub fn create(&self, _data:&Data) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(data_to_document(_data), None)
    }

    // Update an existing document 
    pub fn update(&self, _data:&Data, _param: &String) -> Result<UpdateResult, Error> {
        let object_param = bson::oid::ObjectId::with_string(_param).unwrap();
        self.collection.update_one(doc! { "_id": object_param }, data_to_document(_data), None)
    }

    // Delete some document
    pub fn delete(&self, _carrera: &String) -> Result<DeleteResult, Error> {
        self.collection.delete_one(doc! { "carrera": _carrera }, None)
    }

    // Get all documents
    pub fn get_json(&self) -> std::result::Result<std::vec::Vec<bson::ordered::OrderedDocument>, mongodb::error::Error> {
        let cursor = self.collection.find(None, None).ok().expect("Failed to execute find.");
        let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
        Ok(docs)
    }

    // Get documents with quiery
    pub fn get_by(&self, param: &String) -> std::result::Result<std::vec::Vec<bson::ordered::OrderedDocument>, mongodb::error::Error> {
        let cursor = self.collection.find(doc! { "author": { "$regex": param } }, None).ok().expect("Failed to execute find.");
        let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
        let _serialized = serde_json::to_string(&docs).unwrap();
        Ok(docs)
    }
}
