<script lang="ts">
    import * as THREE from 'three';
    import * as SC from 'svelte-cubed';
    import { onMount } from 'svelte';

    const BLACK = "#000000";
    const WHITE = "#ffffff";
    const YELLOW = "#fff800";
    const GREEN = "#00953a";
    const BLUE = "#005be6";
    const RED = "#ea1b1b";
    const ORANGE = "#ff8500";

    const faceTexture = (color: string = BLACK) => {
        const canvas = document.createElement("canvas");
        canvas.width = 1000;
        canvas.height = 1000;
        const context = canvas.getContext("2d");

        context.fillStyle = "#000000";
        context.fillRect(0, 0, 1000, 1000);
        context.fillStyle = color;
        context.fillRect(50, 50, 900, 900);

        return new THREE.CanvasTexture(canvas);
    };

    let depth = 0.85;
    let width = 0.85;
    let height = 0.85;

    const pieces = [];

    onMount(() => {
        for (let x = -1; x <= 1; x++) {
            for (let y = -1; y <= 1; y++) {
                for (let z = -1; z <= 1; z++) {
                    let top = BLACK;
                    let bottom = BLACK;
                    let front = BLACK;
                    let back = BLACK;
                    let right = BLACK;
                    let left = BLACK;
                    if (x === -1) {
                        left = ORANGE;
                    } else if (x === 1) {
                        right = RED;
                    }
                    if (y === -1) {
                        bottom = YELLOW;
                    } else if (y === 1) {
                        top = WHITE;
                    }
                    if (z === -1) {
                        back = BLUE;
                    } else if (z === 1) {
                        front = GREEN;
                    }

                    const geometry = new THREE.BoxGeometry(width, height, depth);
                    const materials = [/* right, left, top, bottom, front, back */];

                    for (let i = 0; i < 6; i++) {
                        let texture;
                        if (i === 0) {
                            texture = faceTexture(right);
                        } else if (i === 1) {
                            texture = faceTexture(left);
                        } else if (i === 2) {
                            texture = faceTexture(top);
                        } else if (i === 3) {
                            texture = faceTexture(bottom);
                        } else if (i === 4) {
                            texture = faceTexture(front);
                        } else if (i === 5) {
                            texture = faceTexture(back);
                        }
                        const material = new THREE.MeshLambertMaterial({ map: texture });
                        materials.push(material);
                    }

                    pieces.push({ geometry, material: materials, position: [x, y, z]});
                }
            }
        }
    });

    let spin = 0;

    SC.onFrame(() => {
        spin += 0.005;
    });

    const geo = new THREE.BoxGeometry();

    const turnR = (cw = true) => {};
    const turnL = (cw = true) => {};
    const turnU = (cw = true) => {};
    const turnD = (cw = true) => {};
    const turnF = (cw = true) => {};
    const turnB = (cw = true) => {};
    // const turnE = () => {};
    // const turnM = () => {};

    let urfRot = [0, 0, 0];
    let urfPos = [1, 1, 1];
    // setInterval(() => {
    //     urfRot[1] -= Math.PI / 2;
    // }, 1000);
</script>

<!-- <SC.Canvas
    antialias
    background={new THREE.Color('papayawhip')}
    fog={new THREE.FogExp2('papayawhip', 0.05)}
    shadows
> -->
<SC.Canvas
    antialias
    background={new THREE.Color('white')}
    shadows
>

    <SC.Group rotation={[0, spin, 0]}>
        {#each pieces as { geometry, material, position }}
            <SC.Mesh {geometry} {material} {position} />
        {/each}
    </SC.Group>

    <SC.PerspectiveCamera position={[3, 5, 10]} />
    <!-- <SC.OrthographicCamera position={[1, 1, 6]} /> -->
    <SC.AmbientLight intensity={0.6} />
    <SC.DirectionalLight intensity={0.6} position={[-2, 3, 2]} shadow={{ mapSize: [2048, 2048]}} />
    <!-- <SC.OrbitControls enableZoom={true} maxPolarAngle={Math.PI * 0.51} /> -->
    <SC.OrbitControls enableZoom={true} />
</SC.Canvas>