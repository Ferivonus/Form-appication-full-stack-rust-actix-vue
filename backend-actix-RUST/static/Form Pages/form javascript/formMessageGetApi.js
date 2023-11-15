// form javascript/formMessageGetApi.js

async function fetchAndDisplayMessages(formName) {
    const url = `http://localhost:8080/api/messages?form_title=${formName}`;

    try {
        const response = await fetch(url);
        const responseData = await response.json();

        if (response.ok) {
            const messagesContainer = document.getElementById('formMessages');
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