<script>
    import { onMount } from 'svelte';
    import Girlfriend_fav_picture from '$lib/images/sevgili-gorsel.png';

    let contentIndex = 0;
    const contents = [
        "Sevgilim, seni her zaman özlüyorum ve her zaman merak ediyorum. Asla unutamıyorum ve aklımdan çıkaramıyorum.",
        "Biliyorum ki sen de aynı duyguları benim için besliyorsun ve benim için hissediyorsun. Bu yüzden çok mutluyum.",
        "Sen, birçok zor zamanı aşarken yanımda olan sevgilim, hayatımın anlamı ve beni özel hissettiren kişisin. Beni mutlu eden ve hayatıma her gün farklı bir anlam katan sevgilimsin.",
        "Günün her anında seni düşünüyorum ve hayal ediyorum. Seninle geçirdiğimiz her an, özel anılarımızın bir parçası haline geliyor.",
        "İkimiz arasındaki bağ o kadar güçlü ki, hiçbir mesafe ya da zaman bu duyguları değiştiremez. Seninle birlikte olmak her şeye değer.",
        "Seninle hayatın güzelliklerini keşfetmek, seyahat etmek ve yeni maceralar yaşamak istiyorum. Geleceği seninle hayal etmek bile heyecan verici.",
        "Seninle geçirdiğim her anı ve her güzel anı, ömür boyu hatırlayacağım. Seni seviyorum ve her zaman senin için buradayım.",
        "Sevgilim, seninle geçirdiğim her gün benim için bir hazine. Seninle hayatın güzelliklerini keşfetmek, seninle geçirdiğim her anı ömür boyu hatırlamak istiyorum. Sen benim için çok özelsin.",
        "Seni çok seviyorum."
    ];

    let currentPage = contentIndex + 1;

    onMount(() => {
        const contentElement = document.getElementById("content");
        if (contentElement) {
            contentElement.textContent = contents[contentIndex];
        }
    });

    function showNextContent() {
        if (contentIndex < contents.length - 1) {
            contentIndex++;
            currentPage = contentIndex + 1;
            updateContent();
        }
    }

    function showPreviousContent() {
        if (contentIndex > 0) {
            contentIndex--;
            currentPage = contentIndex + 1;
            updateContent();
        }
    }

    function updateContent() {
        const contentElement = document.getElementById("content");
        if (contentElement) {
            contentElement.textContent = contents[contentIndex];
        }
    }
</script>

<svelte:head>
    <title>Sevgilim İçin</title>
    <meta name="description" content="Sevgilim İçin Bir Mesaj" />
    <link href="https://fonts.googleapis.com/css?family=Open+Sans" rel="stylesheet">
</svelte:head>

<div class="text-column">
    <h1>Sevgilim</h1>
    <p id="content"></p>
    <div class="page-navigation">
        <button on:click={showPreviousContent} disabled={contentIndex === 0}>&lt; Geri</button>
        <span>Sayfa {currentPage} / {contents.length}</span>
        <button on:click={showNextContent} disabled={contentIndex === contents.length - 1}>İleri &gt;</button>
    </div>
</div>

<picture>
    <source srcset={Girlfriend_fav_picture} type="image/webp" />
    <img src={Girlfriend_fav_picture} alt="Sevgili Resmi" style="margin-top: 20px;" />
</picture>

<style>
    :root {
        --font-body: 'Open Sans', sans-serif;
        --color-bg-1: hsl(300, 20%, 90%);
        --color-theme-1: #FF3E00;
        --color-theme-2: #4075A6;
        --color-text: #333;
    }

    h1 {
        font-size: 2.2rem;
        text-align: center;
        color: var(--color-theme-1);
        text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.2);
    }

    p {
        font-size: 1.2rem;
        line-height: 1.6;
        text-align: center;
        color: var(--color-text);
        margin-bottom: 20px;
    }

    .page-navigation {
        display: flex;
        justify-content: center;
        align-items: center;
    }

    button {
        background-color: var(--color-theme-1);
        color: white;
        border: none;
        border-radius: 4px;
        padding: 8px 16px;
        margin: 0 10px;
        cursor: pointer;
    }

    button:disabled {
        background-color: #ccc;
        cursor: not-allowed;
    }

    img {
        max-width: 100%;
        height: auto;
        display: block;
        margin: 0 auto;
    }
</style>
