
import { fetch_data } from "./fetch_post.js";


//Обработчик для удаления категории:
export async function delete_data(id, fetch_url){
  let data = {
    "id": id
  };

  try{
    let result = await fetch_data(fetch_url, data);
    return result;
  }catch(err){
    return err;
  }
}
