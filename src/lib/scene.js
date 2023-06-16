import * as THREE from "three";

const scene = new THREE.Scene();
const container = document.getElementById("3dscene");
let w = container.offsetWidth;
let h = container.offsetHeight;
const camera = new THREE.PerspectiveCamera(75, w / h, 0.1, 1000);
const geometry = new THREE.BoxGeometry();
const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
const cube = new THREE.Mesh(geometry, material);
let renderer;
scene.add(cube);
camera.position.z = 5;

const animate = () => {
    requestAnimationFrame(animate);
    cube.rotation.x += 0.01;
    cube.rotation.y += 0.01;
    renderer.render(scene, camera);
};

const resize = () => {
    w = container.offsetWidth;
    h = container.offsetHeight;
    renderer.setSize(w, h);
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
};

export const createScene = (el) => {
    renderer = new THREE.WebGLRenderer({ antialias: true, canvas: el });
    renderer.setSize(w, h);
    resize();
    animate();
};

// window.addEventListener("resize", resize);
