let slideIndex = 1;
let paused = true;
let loop = true;

function plusSlides(n) {
    slideIndex += n;

    if (slideIndex > 3 && !loop) {
        slideIndex = 3;
    }

    if (slideIndex < 1 && !loop) {
        slideIndex = 1;
    }

    showNthSlides(slideIndex);
}

function currentSlide(n) {
    slideIndex = n;
    showNthSlides(slideIndex);
}

function showNthSlides(n) {
    let i;
    let slides = document.getElementsByClassName("mySlides");
    let dots = document.getElementsByClassName("dot");
    if (n > slides.length) {
        slideIndex = 1
    }
    if (n < 1) { slideIndex = slides.length }
    for (i = 0; i < slides.length; i++) {
        slides[i].style.display = "none";
    }
    for (i = 0; i < dots.length; i++) {
        dots[i].className = dots[i].className.replace(" active", "");
    }
    slides[slideIndex - 1].style.display = "block";
    dots[slideIndex - 1].className += " active";
}


function showSlides() {
    if (!paused) {
        let i;
        let slides = document.getElementsByClassName("mySlides");
        for (i = 0; i < slides.length; i++) {
            slides[i].style.display = "none";
        }
        slideIndex++;
        if (slideIndex > slides.length) { slideIndex = 1 }
        slides[slideIndex - 1].style.display = "block";
    }
    setTimeout(showSlides, 2000);
}

document.addEventListener('DOMContentLoaded', () => {
    showNthSlides(1);
    showSlides();
})

