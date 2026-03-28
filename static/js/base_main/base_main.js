import { fetch_get } from "../fetch_get.js";
import { notify } from "../notification_modal.js";
import { event_search } from "./search.js";
import { modal_redact, add_exit_event_for_redact_modal } from "../products/redact_product.js";


async function select_past_categories() {
  let div = document.querySelector(".past-categories-cards");
  let limit = 5;

  try{
    let result = await fetch_get(`api/category/select_categories?limit=${limit}`);

    if(result.data.length > 0){

      result.data.forEach(el => {
        let card = document.createElement("div"); 
        let span_name = document.createElement("span");

        card.classList.add("card");
        span_name.textContent = el.name; 
        card.appendChild(span_name);
        div.appendChild(card);
      });
    }else{
      let span = document.createElement("span");      
      span.textContent = "Категорий нет!"
      div.appendChild(span);
    }
  }catch(err) {
    notify(err, true);
  }
}


async function select_past_products() {
  let div = document.querySelector(".past-products-cards");
  let limit = 5;
  try{
    let result = await fetch_get(`api/products/select_products?limit=${limit}`);
    if(result.data.length > 0){

      result.data.forEach(el => {
        let card = document.createElement("div"); 
        let span_name = document.createElement("span");

        card.classList.add("card");
        span_name.textContent = el.name; 
        card.appendChild(span_name);
        div.appendChild(card);
      });
    }else{
      let span = document.createElement("span");      
      span.textContent = "Товаров нет!"
      div.appendChild(span);
    }

  }catch(err) {
    notify(err, true);
  }
}


document.addEventListener("DOMContentLoaded", () => {
  let redact_btn = document.querySelector('#redact-btn');
  select_past_categories();
  select_past_products();
  event_search();
  //Добавляем обработчик на кнопку submit для внесения изменений
  redact_btn.addEventListener('click', modal_redact);
  //Обработчик для кнопки закрытия окна редактирования продуктов:
  add_exit_event_for_redact_modal();
});


