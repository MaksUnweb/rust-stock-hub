import { fetch_data } from "../fetch_post.js";
import { notify } from "../notification_modal.js";
import { redact_product } from "../products/redact_product.js";
import { delete_data } from "../delete_data.js";

export function event_search() {
  let input = document.querySelector("#search-input");
  let timer;
  input.addEventListener("input", (e) => {
    e.preventDefault();

    clearTimeout(timer);

     timer = setTimeout(() => {
      let search_data = e.target.value;
      if(search_data !== ""){
        search(search_data);
      }
    }, 500)

  });
}

async function search(data) {
  let data_f = {
    "data": data
  }; 

  try{
    let products = await fetch_data("api/products/search", data_f);
  
    let search_preview = document.querySelector(".search-preview");
    search_preview.innerHTML = "";

    if( products.data !== null && products.data.length > 0){
      

      products.data.forEach(el => {
        const svgNS = 'http://www.w3.org/2000/svg';
        const xlinkNS = 'http://www.w3.org/1999/xlink';

        let div_card = document.createElement("div");
        let div_products_id = document.createElement("div");
        let div_products_article = document.createElement("div");
        let div_products_actions = document.createElement("div");

        let span_products_id = document.createElement("span");
        let span_products_article = document.createElement("span");


        let svg_delete = document.createElementNS(svgNS, 'svg');
        let svg_redact = document.createElementNS(svgNS, 'svg');
        let use_delete = document.createElementNS(svgNS, "use");
        let use_redact = document.createElementNS(svgNS, "use");


        div_card.classList.add("card");
        div_products_id.classList.add("products-id");
        div_products_article.classList.add("products-article");
        div_products_actions.classList.add("products-actions");

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
        
        span_products_id.textContent = el.id;
        span_products_article.textContent = el.article;


        div_products_id.appendChild(span_products_id);
        div_products_article.appendChild(span_products_article);
        svg_delete.appendChild(use_delete);
        svg_redact.appendChild(use_redact);
        div_products_actions.appendChild(svg_delete);
        div_products_actions.appendChild(svg_redact);

        div_card.appendChild(div_products_id);
        div_card.appendChild(div_products_article);
        div_card.appendChild(div_products_actions);

        search_preview.appendChild(div_card);

      //Добавляю обработчик нажатия на каждую кнопку:
        //При нажатии запускается определённая функция, в неё передаётся id элемента
        svg_delete.addEventListener("click", async (e) => {
          e.preventDefault();
          
          try{
            let result = await delete_data(el.id, "api/products/delete_product");

            if(result.success){
              notify(result.message, false);
            }else{
              notify(result.message, true);
            }

          }catch(err){
            notify(err, true);
          }

        });

        svg_redact.addEventListener("click", (e) => {
        e.preventDefault();
          redact_product(el.id);
        });

      });
    }else{
        let div_card = document.createElement("div");
        let span = document.createElement("span");

        div_card.classList.add("card-null");
        span.textContent = products.message;
      div_card.appendChild(span);
      search_preview.appendChild(div_card);
    }

  }catch(err){
      notify(err, true);
  }
}
