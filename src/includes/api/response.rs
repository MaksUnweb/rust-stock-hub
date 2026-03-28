use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Response<T> {
    success: bool,
    message: String,
    data: Option<Vec<T>>
} 


impl<T> Response<T> {
    //Метод для успешного ответа с данными:
    pub fn ok(data: Vec<T>, message: String) -> Self {
        Self { 
        success: true,
        message: message.to_string(),
        data: Some(data)
        }
    }

    //Метод для успешного ответа без данных:
    pub fn ok_emty(message: String) -> Self {
        Self { 
            success: true,
            message: message.to_string(),
            data: None
        }
    }

    //Метод для возващения ошибки:
    pub fn err(message: String) -> Self {
        Self { 
            success: false, 
            message: message.to_string(),
            data: None
        }
    }
}
