<script>

	let isim = '';
	let yaş = parseInt("0");
	let hikaye = ''; // Giriş alanının değerini tutmak için bir değişken oluşturun
</script>
<svelte:head>
	<title>About</title>
	<meta name="description" content="About this app" />
</svelte:head>

<div class="text-column">

	<h2>bizim bir hikayemiz var:</h2>
	<div class="form-container">
		<input type="text" class="input-field" bind:value={isim} placeholder="isminizi girin">
		<p class="result">adınız: {isim}</p>
	</div>
	<div class="form-container">
		<input type="text" class="input-field" bind:value={yaş} placeholder="yaşınızı girin">
		<p class="result">yaşınız: {yaş}</p>
	</div><div class="form-container">
		<input type="text" class="input-field" bind:value={hikaye} placeholder="hikayenizi girin">
		<p class="result">hikayeniz: {hikaye}</p>
	</div>
	<div class="hikaye">
		<p>Merhaba, benim adım {isim}, yaşım da {yaş}</p>
		<p>Benim hikayem: {hikaye}</p>
	</div>
	<p>tuğçe merhaba</p>
	<p>tuğçe okulan gitti</p>
	<p>
		tuğçe kuran kursuna gittis
	</p>



	<h1>About this app</h1>s

	<p>
		This is a <a href="https://kit.svelte.dev">SvelteKit</a> app. You can make your own by typing the
		following into your command line and following the prompts:
	</p>

	<pre>npm create svelte@latest</pre>

	<p>
		The page you're looking at is purely static HTML, with no client-side interactivity needed.
		Because of that, we don't need to load any JavaScript. Try viewing the page's source, or opening
		the devtools network panel and reloading.
	</p>

	<p>
		The <a href="/sverdle">Sverdle</a> page illustrates SvelteKit's data loading and form handling. Try
		using it with JavaScript disabled!
	</p>
</div>
<style>
	/* form-styles.css */
.form-container {
  text-align: center;
  margin: 20px;
  padding: 20px;
  border: 1px solid #ccc;
  border-radius: 5px;
  background-color: #f0f0f0;
}

.input-field {
  width: 100%;
  padding: 10px;
  margin: 10px 0;
  border: 1px solid #999;
  border-radius: 5px;
}

.result {
  font-size: 18px;
  margin-top: 10px;
}

</style>