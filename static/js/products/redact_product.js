import { notify } from "../notification_modal.js";
import { fetch_data } from "../fetch_post.js";
import { fetch_get } from "../fetch_get.js";

//Функция для открытия окна редактирования продукта,
//также заполняет форму старыми данными, для возможности редактирования 
//Здесь есть момент с выводом текущей категории, дело в том, что все данные, даже если вывод из бд происходил по id (то есть уникально),
//выводятся в виде вектора (в случае JS - массива), по этому даже если в массиве лишь один элемент, я всё равно прохожусь по нему циклом,
//также можно сделать обращение к элементу массива с помощью индекса (он всегда будет 0 так как товар по ID выводится лишь один),
  //если у кого-то есть идеи, как оптимизировать этот код, то сделайте это :)
export async function redact_product(id) {
  let modal = document.querySelector(".modal-redact");
  modal.style.display = "flex";

  //Заполняем поля формы редактирования данными:
  try{
    
    let current_data = await fetch_get(`api/products/select_products?id=${id}`);
    let all_categories = await fetch_get("api/category/select_categories");


    let article_number = modal.querySelector("#redact-article-number");
    let title_product = modal.querySelector("#redact-title-product");
    let category = modal.querySelector("#redact-category");
    let redact_quantity_product = modal.querySelector("#redact-quantity-product");
    let redact_price_product = modal.querySelector("#redact-price-product");
    let product_id = modal.querySelector("#product-id");

    category.innerHTML = "";
    // Создаю переменную для хранения текущей категории:
    let current_category;
    if(current_data.data.length > 0){
    
        current_data.data.forEach(el => {
          article_number.setAttribute("value", el.article);
          title_product.setAttribute("value", el.name);

        redact_quantity_product.setAttribute("value", el.quantity);
        redact_price_product.setAttribute("value", el.price);
        product_id.value = id;
          current_category = el.category_id;
        });


      //Создаём пустую категорию:
      let null_option = document.createElement("option");
      null_option.textContent = "Без категории";
      null_option.setAttribute("value", null)
      category.appendChild(null_option);
 
      if(all_categories.success) {
        all_categories.data.forEach(cat => {
          
        let option = document.createElement("option");
        option.textContent = cat.name;
        option.setAttribute("value", cat.id);
        category.appendChild(option);
        //Ставим текущую категорию:
        if(current_category === cat.id){
          category.value = cat.id;
        }
        });
      }else{
        notify(current_data.message, true);
      }
    }else {
      notify(current_data.message, true);
    }


  }catch(err){
    modal.style.display = "none";
    notify(err, true);
  }
}


//Функция для работы с модальным окном редактирования:
export async function modal_redact(e) {
  e.preventDefault();

  let redact_article_number = document.querySelector("#redact-article-number").value;
  let redact_title_product = document.querySelector("#redact-title-product").value;
  let redact_category = document.querySelector("#redact-category").value;
  let redact_quantity_product = document.querySelector("#redact-quantity-product").value;
  let redact_price_product = document.querySelector("#redact-price-product").value;
  let product_id = document.querySelector("#product-id").value;

  //Также беру само модальное окно, чтобы закрыть его при обновлении:
  let modal = document.querySelector(".modal-redact");


    let category_id;

    //Отправляем изменения на сервер:
    if(redact_category !== null || redact_category !== undefined || redact_category !== NaN){
       category_id = Number(redact_category);
    }else{
       category_id = null;
    }

    let quantity = Number(redact_quantity_product);
    let price = Number(redact_price_product);

    let data = {
      "id": Number(product_id),
      "article_number": redact_article_number,
      "title_product": redact_title_product,
      "category_id": category_id,
      "quantity": quantity,
      "price":  price 
    };

    //Добавляем обработчик для загрузки редактирования:
      try{
        let result = await fetch_data("api/products/update_product", data);
        notify(result.message, false);
        modal.style.display = "none";
      }catch(err){
        notify(err, true);
      }
}  


//Функция для добавления обработчика на кнопку 
//закрытия окна редактирования:
export function add_exit_event_for_redact_modal() {
  let modal = document.querySelector(".modal-redact");
  let btn = modal.querySelector("#exit-btn");
    btn.addEventListener("click", (e) => {
    e.preventDefault();
    modal.style.display = "none";
  });
}
