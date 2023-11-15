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