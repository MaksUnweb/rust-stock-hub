//Здесь располагается код для обработки добавления новых товаров в хранлище
import { event_pagination_btn } from "./select_all_products.js";
import { fetch_data } from "../fetch_post.js";
import { notify } from "../notification_modal.js";


  
//Функция для добавления обработчика на кнопку submit формы добавления продуктов;
//после того, как обработчик отработает, данные из формы будут отправлены на сервер:
export function add_event_form() {
  let form = document.querySelector("#add-product-form");
  form.addEventListener("submit", async (e) => {
    e.preventDefault();

    let article_number = form.querySelector("#article-number").value;
    let title_product = form.querySelector("#title-product").value;
    let category = form.querySelector("#category").value;
    let quantity_product = form.querySelector("#quantity-product").value;
    let price_product = form.querySelector("#price-product").value;

    quantity_product = Number(quantity_product);
    price_product = Number(price_product);
    
    let data = {
      "article_number": article_number,
      "title_product": title_product,
      "category_product": category,
      "quantity_product": quantity_product,
      "price_product": price_product
    };

    //Создаём запрос:

    try {
      let result = await fetch_data("api/products/add_product", data);

      if(result.success == true){
        notify(result.message, false);
        event_pagination_btn();
      }else{
        notify(result.message, true);
      }
    }catch (err) {
        notify(err, true);
    }
  });
}

