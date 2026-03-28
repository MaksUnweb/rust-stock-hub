import { fetch_data } from "../fetch_post.js";


//Обработчик для удаления категории:
export async function delete_category(id){
  let data = {
    "id": id
  };

  try{
    let result = await fetch_data("api/categories/delete_category", data);
    return result;
  }catch(err){
    return err;
  }
}
