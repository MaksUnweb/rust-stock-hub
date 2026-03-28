

function login() {
  let form = document.querySelector("#login-form");
  form.addEventListener("submit", (e) => {
    e.preventDefault();

    let login = form.querySelector("#admin-login").value;
    let password = form.querySelector("#admin-password").value;
  
    let data = {
      "admin_login": login,
      "admin_password": password
    };

    fetch_login(data);
  })
}


function fetch_login(data) {
  fetch("/api/login", {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(data)
  })
    .then(response => {
      if (!response.ok){
        throw new Error("Ошибка отправки запроса!");
      }
      return response.json();
    })
    .then(result => {

      if(result.success === true) {
        window.location.href = '/'
      }else{
        throw new Error(`Ошибка: ${result.message}`);
      }

    })
    .catch(error => {
     alert(error); 
    }) 
    
}


document.addEventListener("DOMContentLoaded", () => {
  login();
})
