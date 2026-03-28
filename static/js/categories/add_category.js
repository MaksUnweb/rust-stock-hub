import { event_pagination } from "./select_all_categories.js";
import { fetch_data } from "../fetch_post.js";
import { notify } from "../notification_modal.js";


export async function add_category() {

  let form = document.querySelector("#add-category-form");
  
  form.addEventListener("submit", async (e) => {
    e.preventDefault();
    
    let category_name = form.querySelector("#category-name").value; 
    let parent_id = form.querySelector("#parent-id").value;

    let data = {
      "name": category_name,
      "parent_id": parent_id
    };


    try {
      let result = await fetch_data("api/categories/add_category", data);
      notify(result.message, false);
      event_pagination();
    }catch(err){
      notify(err, true);
    }

  });

}

