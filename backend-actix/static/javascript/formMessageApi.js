// form javascript/formMessageGetApi.js

// Fetch messages for the specified form
async function fetchAndDisplayMessages(form_title) {
    const url = `http://localhost:8080/api/messages/${form_title}`;

    try {
        const response = await fetch(url);
        const responseData = await response.json();

        if (response.ok) {
            const messagesContainer = document.getElementById('formMessages');
            messagesContainer.innerHTML = ''; // Clear previous messages

            responseData.messages.forEach((message) => {
                const listItem = document.createElement('li');
                const formattedDate = message.createdAt ? new Date(message.createdAt).toLocaleString() : 'N/A';
                const updatedDate = message.updatedAt ? new Date(message.updatedAt).toLocaleString() : 'N/A';
            
                const visibility = message.published ? 'Public' : 'Private';
            
                listItem.innerHTML = `
                    <article class="message">
                        <h4 class="message-title">${message.title}</h4>
                        <p class="message-content">${message.content}</p>
                        <div class="message-details">
                            <em class="message-author">By: ${message.id || 'Anonymous'}</em>
                            <br>
                            <em class="message-visibility">Published: ${visibility}</em>
                            <br>
                            <em class="message-date">Created At: ${formattedDate}</em>
                            <br>
                            <em class="message-date">Last Update At: ${updatedDate}</em>
                        </div>
                    </article>
                `;
            
                messagesContainer.appendChild(listItem);
            });
            
            
        } else {
            console.error('Error:', responseData);
        }
    } catch (error) {
        console.error('Fetch error:', error);
    }
}


async function AddFormTitleMessage(form_title) {
    const url = 'http://localhost:8080/api/messages/';

    const messageData = {
        title: document.getElementById('title').value,
        content: document.getElementById('content').value,
        form_title: form_title,
        published: true,
    };

    try {
        const response = await fetch(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(messageData),
        });

        if (response.ok) {
            const responseData = await response.json();
            console.log('Success:', responseData);
            fetchAndDisplayMessages(form_title);  // Fetch messages after creating a new one
        } else {
            const errorData = await response.json();
            console.error('Error:', errorData);
        }
    } catch (error) {
        console.error('Fetch error:', error);
    }
}

async function fetchAndDisplayAllMessages() {
    const url = 'http://localhost:8080/api/messages';

    try {
        const response = await fetch(url);
        const responseData = await response.json();

        if (response.ok) {
            const messagesContainer = document.getElementById('MessagesList');
            messagesContainer.innerHTML = ''; // Clear previous messages

            responseData.messages.forEach((message) => {
                const listItem = document.createElement('li');
                listItem.textContent = `${message.title}: ${message.content}`;
                messagesContainer.appendChild(listItem);
            });
        } else {
            console.error('Error:', responseData);
        }
    } catch (error) {
        console.error('Fetch error:', error);
    }
}


async function findUserByUsernameAndPassword() {

    const user ={
        username: document.getElementById('username-login').value,
        password: password = document.getElementById('password-login').value,
    }

    // http://localhost:8080/api/users/check?username=john_doe&password=hashed_password_123
    const url = `http://localhost:8080/api/user/info/by_username`;

    try {
        const response = await fetch(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(user),
        });

        const data = await response.json();

        if (data.status === 'success') {
            const userListDiv = document.getElementById('userList-usernameAndPassword');
            userListDiv.innerHTML = `<h2>Users (${data.results} found)</h2>`;

            data.users.forEach(user => {
                const userElement = document.createElement('div');
                userElement.innerHTML = `
                    <p>ID: ${user.id}</p>
                    <p>Username: ${user.username}</p>
                    <p>Email: ${user.email}</p>
                    <p>Registration Date: ${user.registration_date || 'N/A'}</p>
                    <hr>
                `;
                userListDiv.appendChild(userElement);
            });
        } else {
            console.error('Error:', data.message);
        }
    } catch (error) {
        console.error('Fetch error:', error);
    }
}

async function findUserByMailAndPassword() {

    const user ={
        email: document.getElementById('mail-login').value,
        password: document.getElementById('password-login-with-mail').value,
    }

    // http://localhost:8080/api/users/check?username=john_doe&password=hashed_password_123
    const url = `http://localhost:8080/api/user/info/by_email`;

    try {
        const response = await fetch(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(user),
        });

        const data = await response.json();

        if (data.status === 'success') {
            const userListDiv = document.getElementById('userList-MailAndPassword');
            userListDiv.innerHTML = `<h2>Users (${data.results} found)</h2>`;

            data.users.forEach(user => {
                const userElement = document.createElement('div');
                userElement.innerHTML = `
                    <p></p>
                    <p>ID: ${user.id}</p>
                    <p>Username: ${user.username}</p>
                    <p>Email: ${user.email}</p>
                    <p>Registration Date: ${user.registration_date || 'N/A'}</p>
                    <hr>
                `;
                userListDiv.appendChild(userElement);
            });
        } else {
            console.error('Error:', data.message);
        }
    } catch (error) {
        console.error('Fetch error:', error);
    }
}

function registerUser() {

    const apiUrl = 'http://localhost:8080/api/user'; // Replace with the actual URL where your Rust API is running

    const user = {
        username: document.getElementById('Registerusername').value,
        password: document.getElementById('Registerpassword').value,
        email: document.getElementById('Registeremail').value,
    }

    const requestOptions = {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(user),
    };

    fetch(`${apiUrl}/register/`, requestOptions)
        .then(response => response.json())
        .then(data => {
            console.log('Response:', data);
        })
        .catch(error => {
            console.error('Error:', error);
        });

    }
