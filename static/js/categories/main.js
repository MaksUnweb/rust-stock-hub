import { select_in_form } from "./select_in_form.js";
import { add_category } from "./add_category.js";
import { event_pagination } from "./select_all_categories.js";
import { modal_redact, event_exit_redact_btn } from "./redact_category.js";






document.addEventListener("DOMContentLoaded", () => {
  let selector = document.querySelector("#parent-id");
  let next_pagination_btn = document.querySelector("#next-btn");
  let back_pagination_btn = document.querySelector("#back-btn");
  let redact_btn = document.querySelector("#redact-btn");
  //Выводим категории в форму:
  select_in_form(selector);
  //Ставим обработчик на добавление новых категорий:
  add_category();
  //Выводим все категории на шаблон:
  event_pagination();
  //Метод для модального окна:
  redact_btn.addEventListener("click", modal_redact);
  //Метод для обработки кнопки закрытия модального окна:
  event_exit_redact_btn();
})
