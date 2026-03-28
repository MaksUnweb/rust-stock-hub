

export function notify(msg, isError) {
  let modal = document.querySelector(".modal-result");
  modal.innerHTML = "";
  let p = document.createElement("p");
  p.textContent = msg;
  
  modal.appendChild(p);

  if(isError === true) {
    modal.classList.add("error-result");
  }else{
    modal.classList.add("success-result");
  }
  
  open_modal(modal);
  setTimeout(() => {
   close_modal(modal);
  }, 900)
}


function open_modal(modal) {
  modal.style.display = "flex";
  let opacity = 0;
  let prevTime = performance.now();
   
  function animate(time) {
    const delta = (time - prevTime) / 1000;
    opacity += 4 * delta;
  
    if (opacity > 1) opacity = 1;

    modal.style.opacity = opacity;
    prevTime = time;

    if (opacity < 1){
      requestAnimationFrame(animate);
    }
  }

  requestAnimationFrame(animate);
}


function close_modal(modal) {
    let opacity = 1;
  let prevTime = performance.now();

  function animate(time) {
    const delta = (time - prevTime) / 1000;
    opacity -= 4 * delta;

    if(opacity < 0) {
      opacity = 0;
      modal.style.display = "none";
      if(modal.classList.contains("error-result")){
       modal.classList.remove("error-result");
      }else if(modal.classList.contains("success-result")) {
        modal.classList.remove("success-result");
      }
    }

    modal.style.opacity = opacity;
    prevTime = time;

    if (opacity > 0){
      requestAnimationFrame(animate);
    }
    
  }

  requestAnimationFrame(animate);
}
