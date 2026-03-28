


//Function for add events on the buttons management modal window:
function headerAction() {
  let header = document.querySelector("header");
  let open_modal = header.querySelector("#open-modal");
  let close_modal = header.querySelector("#close-modal");
  let modal = header.querySelector(".header-modal");

  open_modal.addEventListener("click", (e) => {
    open_header_modal(modal);
  });

  close_modal.addEventListener("click", (e) => {
    close_header_modal(modal);
  });

}


/*
function open_header_modal(modal) {
  modal.style.display = "flex";
  let window_width = document.documentElement.clientWidth;
  let current_position = window_width;
  modal.style.transform = `translateX(${window_width + 1}px)`;

  let timer = setInterval(() => {
    current_position -= 38.4; 
    modal.style.transform = `translateX(${current_position}px)`
    if(current_position <= 0) {
      clearInterval(timer);
    }
    
  }, 15)
}
*/

function open_header_modal(modal) {
  modal.style.display = "flex";
  let window_width = document.documentElement.clientWidth;
  let start_position = ( window_width / 2);
  let duration = 700; 
  let start = performance.now();

  function easeOutQuad(timeFraction) {
    return 1 - Math.pow(1 - timeFraction, 2);
  }


  function animate(time) {
    let timeFraction = (time - start) / duration;
    if (timeFraction > 1) timeFraction = 1;

    let progress = easeOutQuad(timeFraction);
    let current_position = start_position * (1 - progress);
    
    modal.style.transform = `translateX(${current_position}px)`;

    if (timeFraction < 1) {
      requestAnimationFrame(animate);
    }
  }

  requestAnimationFrame(animate);
}



function close_header_modal(modal) {
  let window_width = document.documentElement.clientWidth;
  let start_position = window_width / 2;
  let duration = 700;
  let start = performance.now();

  function easeOutQuad(timeFraction) {
    return 1 - Math.pow(1 - timeFraction, 2);
  }

  function animate(time) {
    let timeFraction = (time - start) / duration;
    if(timeFraction > 1) timeFraction = 1;

    let progress = easeOutQuad(timeFraction);
    let current_position = start_position * progress;

    modal.style.transform = `translateX(${current_position}px)`;

    if(timeFraction < 1) {
      requestAnimationFrame(animate);
    }else{
      modal.style.display = "none";
    }
  }
  requestAnimationFrame(animate);
}

document.addEventListener("DOMContentLoaded", () => {
  headerAction();
})
