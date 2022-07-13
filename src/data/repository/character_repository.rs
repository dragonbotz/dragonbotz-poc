
// lib
    // serenity
use serenity::async_trait;

    // tokio-postgres
use tokio_postgres;

// crate
use crate::data::models::character_model::CharacterModel;
use crate::utils::error::Error;


pub struct CharacterRepository<'a> {
    database: &'a tokio_postgres::Client,
}

impl<'a> CharacterRepository<'a> {

    /// Returns an instance of CharacterRepository
    /// 
    /// ## Arguments:
    /// * database - the database client
    pub fn new(database: &'a tokio_postgres::Client) -> Self {
        Self {
            database,
        }
    }

}

#[async_trait]
pub trait CharacterRepositoryTrait {

    /// Returns a character according to the character id passed as argument
    /// 
    /// ## Argument:
    /// * id - the character's id
    async fn get_character_with_id(self: &Self, id: i32) 
        -> Result<CharacterModel, Error>;

}

#[async_trait]
impl CharacterRepositoryTrait for CharacterRepository<'_> {

    async fn get_character_with_id(&self, id: i32) 
        -> Result<CharacterModel, Error> {

        let result = self.database
            .query_one(
                "SELECT *
                FROM character
                WHERE id = $1::INT4", 
                &[&id]
            ).await;

        if let Err(error) = result {
            return Err(Error::DatabaseQueryError(format!("{}", error)));
        }

        let row = result.unwrap();

        Ok(
            CharacterModel::new(
                row.get(0), row.get(1), row.get(2), row.get(3), row.get(4)
            )
        )
    }

}