import { notify } from "../notification_modal.js";
import { fetch_get } from "../fetch_get.js";
import { delete_data } from "../delete_data.js";
import { redact_category } from "./redact_category.js";

//Добавляем обработчик для кнопок пагинаций
export async function event_pagination() {
  let offset = 0;

  select_all_categories(offset);

  let next_btn = document.querySelector("#next-btn");
  let back_btn = document.querySelector("#back-btn");


  next_btn.addEventListener("click", (e) => {
      e.preventDefault();
    offset += 5;
    select_all_categories(offset);
  });

  back_btn.addEventListener("click", (e) => {
      e.preventDefault();

    if(offset <= 0){
      offset = 0;
    }else{
      offset -= 5;
    }
    select_all_categories(offset);
  });
}


//Функция для вывода всех категорий
async function select_all_categories(offset) {
  let cards_categories = document.querySelector(".cards-categories");
  cards_categories.innerHTML = "";

  if(offset == null || undefined || NaN){
    offset = 0;
  }
  
  let limit = 10;

  try{
    let categories = await fetch_get(`api/category/select_categories?limit=${limit}&offset=${offset}`);

    //Выводим категории в DOM:
    if(categories.data !== null && categories.data.length > 0){

      categories.data.forEach(el => {
        const svgNS = 'http://www.w3.org/2000/svg';
        const xlinkNS = 'http://www.w3.org/1999/xlink';

        let div_card = document.createElement("div");
        let div_category_id = document.createElement("div");
        let div_category_name = document.createElement("div");
        let div_actions = document.createElement("div");

        let span_id = document.createElement("span");
        let span_name = document.createElement("span");
          
        let svg_delete = document.createElementNS(svgNS, 'svg');
        let svg_redact = document.createElementNS(svgNS, 'svg');
        let use_delete = document.createElementNS(svgNS, "use");
        let use_redact = document.createElementNS(svgNS, "use");

        div_card.classList.add("card");
        div_category_id.classList.add("category-id");
        div_category_name.classList.add("category-name");
        div_actions.classList.add("actions");

        svg_delete.setAttribute("width", "16");
        svg_delete.setAttribute("height", "16");
        svg_delete.setAttribute("viewBox", "0 0 16 16");
        use_delete.setAttributeNS(xlinkNS, 'href', '/static/icons/sprite.svg#trash-logo');


        svg_redact.setAttribute("width", "16");
        svg_redact.setAttribute("height", "16");
        svg_redact.setAttribute("viewBox", "0 0 16 16");
        use_redact.setAttributeNS(xlinkNS, 'href', '/static/icons/sprite.svg#redact-logo');

        svg_delete.classList.add("delete-btn");
        svg_redact.classList.add("redact-btn");

        span_id.textContent = el.id;
        span_name.textContent = el.name;

        div_category_id.appendChild(span_id);
        div_category_name.appendChild(span_name);

        svg_delete.appendChild(use_delete);
        svg_redact.appendChild(use_redact);

        div_actions.appendChild(svg_delete);
        div_actions.appendChild(svg_redact);

        div_card.appendChild(div_category_id);
        div_card.appendChild(div_category_name);
        div_card.appendChild(div_actions);
        
        cards_categories.appendChild(div_card);

      //Добавляю обработчик нажатия на каждую кнопку:
        //При нажатии запускается определённая функция, в неё передаётся id элемента
        svg_delete.addEventListener("click", async (e) => {
          e.preventDefault();
          try{
            let result = await delete_data(el.id, "api/categories/delete_category");
            if(result.success){
              notify(result.message, false);
              select_all_categories();
            }else{
              notify(result.message, false);
            }
          }catch(err) {
            notify(err, true);
          }
        });

        svg_redact.addEventListener("click", (e) => {
        e.preventDefault();
          redact_category(el.id);
        });
      });

    }else{
     let div_null = document.createElement("div"); 
      let span_null = document.createElement("span");
      span_null.textContent = "Категорий нет!";
      div_null.classList.add("card-null");
      div_null.appendChild(span_null);
      cards_categories.appendChild(div_null);
    }
  }catch(err){
    notify(err, true);
  }
}

