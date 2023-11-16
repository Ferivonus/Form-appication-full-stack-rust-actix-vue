
async function registerUser() {

    const apiUrl = 'http://localhost:8080/api/user/add/'; // Replace with the actual URL where your Rust API is running

    const user = {
        email: document.getElementById('Registeremail').value,
        username: document.getElementById('Registerusername').value,
        password: document.getElementById('Registerpassword').value,
    };

    const requestOptions = {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(user),
    };

    await fetch(apiUrl, requestOptions)
        .then(response => response.json())
        .then(data => {
            console.log('Response:', data);
        })
        .catch(error => {
            console.error('Error:', error);
        });
}
