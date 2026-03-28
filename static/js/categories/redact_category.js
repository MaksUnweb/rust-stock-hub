import { fetch_get } from "../fetch_get.js";
import { fetch_data } from "../fetch_post.js";
import { notify } from "../notification_modal.js";
import { event_pagination } from "./select_all_categories.js";



//Обработчик для редактирования категории:
export async function redact_category(id){

  let modal = document.querySelector(".modal-redact");
  let redact_category_name = modal.querySelector("#redact-category-name");
  let redact_parent_id = modal.querySelector("#redact-parent-id");
  let redact_category_id = modal.querySelector("#redact-category-id");



  modal.style.display = "flex";

  try{
    let data_output = await fetch_get(`api/category/select_categories?id=${id}`);

    data_output.data.forEach(el => {
      
      redact_category_name.value = el.name;
      let null_option = document.createElement("option");
      null_option.textContent = "Без категории";
      null_option.setAttribute("value", null);
      redact_parent_id.appendChild(null_option);

      let option = document.createElement("option");

      //Очищаем option перед внесением новых значений:
      option.innerHTML = "";

      option.setAttribute("value", el.id);
      option.textContent = el.name
      redact_parent_id.appendChild(option);

      if(el.id === id){
        redact_parent_id.value = el.parent_id;
        redact_category_id.value = el.id;
      }
    });

  }catch(err){
    notify(err, true);
  }
}

//Этот метод запускается из main.js, необходим для обработки кнопки submit в форме редактирования
//По факту здесь я беру данные из формы редактирования и отправляю запрос на сервер
export async function modal_redact(e) {
  e.preventDefault();

  let redact_category_name = document.querySelector("#redact-category-name").value;
  let redact_parent_id = document.querySelector("#redact-parent-id").value;
  let redact_category_id = document.querySelector("#redact-category-id").value;

  if(redact_parent_id !== null || redact_parent_id !== "null" || redact_parent_id !== undefined || redact_parent_id !== NaN){
    redact_parent_id = Number(redact_parent_id);
  }else{
    redact_parent_id = null;
  }


  let data = {
      "id": Number(redact_category_id),
      "name": redact_category_name,
      "parent_id": redact_parent_id
  };

  try{
    let result = await fetch_data("api/categories/update_category", data);
    notify(result.message, false);
    event_pagination(); 
    close_redact_modal();
  }catch(err){
    notify(err, true);
    close_redact_modal();
  }
}


export function event_exit_redact_btn() {
  let exit_btn = document.querySelector("#exit-btn");
  exit_btn.addEventListener("click", (e) => {
    e.preventDefault();
    close_redact_modal();
  })
}

function close_redact_modal() {
  let modal = document.querySelector(".modal-redact");  
  modal.style.display = "none";
}
