//This template shows the output of the category in the template of the category addition form.
  
  import { fetch_get } from "../fetch_get.js";
  import { notify } from "../notification_modal.js";


export async function select_in_form(selector) {
  

  try{
  let categories = await fetch_get("api/category/select_categories");

    if(categories.success){
      if(categories.data !== null){
        let null_option = document.createElement("option");
        null_option.textContent = "Без категории";
        null_option.setAttribute("value", "");
        selector.appendChild(null_option);

        categories.data.forEach(el => {

        let option = document.createElement("option");
          option.textContent = el.name;
          option.setAttribute("value", el.id);
          selector.appendChild(option);
        });
      }else{
        let option = document.createElement("option");
        option.setAttribute("value", null);
        option.textContent = "Без категории"
        selector.appendChild(option);
      }
    }else{
    console.log(categories.message)
      notify(categories.message, true);
    }
  }catch(err){
    notify(err, true);
  }
}

