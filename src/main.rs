use rand::distributions::{Uniform,Distribution};
use rand::Rng;
use std::collections::HashMap;
use serde_json;

use reqwest::Error;
use tokio;

struct LLM<'a>{
    name: &'a str,
    description: Option<&'a str>,
    model_name: &'a str,
    host: &'a str,
    port: i32
}


impl LLM<'_>{
    fn full_url(&self) -> String{
        format!("http://{}:{}",self.host,self.port )
    }
    async fn connect(&self) -> bool{
        let url = self.full_url();
        let res = reqwest::get(url).await.expect("Expected response");
        let status_code = res.status();
        if status_code == 200{
            let data = res.text().await.expect("Expected data in response");
            if &data == "Ollama is running"{
                return true ;
            }
        }
        return false;
    }
    async fn ask_question(&self, question: String)-> Result<String,Error>{
        let client = reqwest::Client::new();
        let body = HashMap::from(
            [
                ("model".to_string(), self.model_name.to_string()),
                ("prompt".to_string(), question)
            ]
        );
        let url = format!("{}/api/generate",self.full_url());
        loop {
            let res = client.post(&url).json(&body).send().await?;
            if res.status() != 200{
                dbg!("Break!!");
                break;
            }
            let data= res.text().await.expect("Expected json body");
            data.split("\n").for_each(|x|{
                println!("{:?}",x);
                let json_data : serde_json::Value = serde_json::from_str(x).expect("");
                dbg!(json_data);
            });
        }
        Ok("".to_string())
    }
}

#[tokio::main]
async fn main() {
    let mut rng = rand::thread_rng();
    let llama_model = LLM{
        name:"LLama",
        description:None,
        model_name:"llama2",
        host:"localhost",
        port:11434
    };

    assert!(llama_model.connect().await,"Can't connect to the LLM server");
    let prompts  = Vec::from([
        "Hi there, you're my personal assistant. Now tell me whether you can work with api ",
        "Can you scan my computer's local file system ? ",
        "what is 23+90*7?",
        "what are two types of door motions ? ",
        "name all the punctuations present in english language"
    ]);

    let i= rng.gen_range(0..prompts.len()-1) as usize;
    llama_model
        .ask_question(prompts[i].to_string())
        .await
        .expect("Expected response");
}
