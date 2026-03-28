import { select_in_form } from "../categories/select_in_form.js";
import { add_event_form } from "./add_product.js";
import { event_pagination_btn } from "./select_all_products.js";
import { modal_redact, add_exit_event_for_redact_modal } from "./redact_product.js";


document.addEventListener("DOMContentLoaded", () => {

    let selector = document.querySelector('#category');
    let redact_btn = document.querySelector('#redact-btn');
    //Выводим категории в форму:
    select_in_form(selector);
    //Обработчик для кнопки submit в форме добавления продуктов
    add_event_form();
    //Обработчик для кнопок пагинации (необходимо для вывода всех продуктов):
    event_pagination_btn();
    //Добавляем обработчик на кнопку submit для внесения изменений
    redact_btn.addEventListener('click', modal_redact);
    //Обработчик для кнопки закрытия окна редактирования продуктов:
    add_exit_event_for_redact_modal();
})