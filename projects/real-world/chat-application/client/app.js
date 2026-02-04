class ChatClient {
    constructor() {
        this.ws = null;
        this.username = null;
        this.currentRoom = null;
        this.connected = false;
        
        this.initElements();
        this.attachEventListeners();
    }

    initElements() {
        this.connectBtn = document.getElementById('connectBtn');
        this.messageForm = document.getElementById('messageForm');
        this.messageInput = document.getElementById('messageInput');
        this.messagesDiv = document.getElementById('messages');
        this.roomsList = document.getElementById('roomsList');
        this.currentRoomDisplay = document.getElementById('currentRoom');
        this.usernameDisplay = document.getElementById('usernameDisplay');
    }

    attachEventListeners() {
        this.connectBtn.addEventListener('click', () => this.toggleConnection());
        this.messageForm.addEventListener('submit', (e) => this.sendMessage(e));
        this.createDefaultRooms();
    }

    createDefaultRooms() {
        const rooms = ['general', 'random', 'tech'];
        rooms.forEach(room => this.addRoomToList(room));
    }

    toggleConnection() {
        if (this.connected) {
            this.disconnect();
        } else {
            this.connect();
        }
    }

    connect() {
        this.ws = new WebSocket('ws://127.0.0.1:9001');

        this.ws.onopen = () => {
            this.connected = true;
            this.connectBtn.textContent = 'Disconnect';
            this.connectBtn.classList.add('connected');
            this.addSystemMessage('Connected to server');
            
            if (!this.username) {
                const name = prompt('Enter your username:');
                if (name) {
                    this.setUsername(name);
                }
            }
        };

        this.ws.onmessage = (event) => {
            const data = JSON.parse(event.data);
            this.handleMessage(data);
        };

        this.ws.onerror = (error) => {
            console.error('WebSocket error:', error);
            this.addSystemMessage('Connection error');
        };

        this.ws.onclose = () => {
            this.connected = false;
            this.connectBtn.textContent = 'Connect';
            this.connectBtn.classList.remove('connected');
            this.connectBtn.classList.add('disconnected');
            this.messageInput.disabled = true;
            this.messageForm.querySelector('button').disabled = true;
            this.addSystemMessage('Disconnected from server');
        };
    }

    disconnect() {
        if (this.ws) {
            this.ws.close();
        }
    }

    handleMessage(data) {
        switch (data.type) {
            case 'Message':
                this.addMessage(data.username, data.content, data.timestamp);
                break;
            case 'PrivateMessage':
                this.addPrivateMessage(data.from, data.content, data.timestamp);
                break;
            case 'SystemMessage':
                this.addSystemMessage(data.content);
                break;
            case 'UserJoined':
                this.addSystemMessage(`${data.username} joined ${data.room}`);
                break;
            case 'UserLeft':
                this.addSystemMessage(`${data.username} left ${data.room}`);
                break;
        }
    }

    setUsername(name) {
        this.username = name;
        this.usernameDisplay.textContent = `👤 ${name}`;
        this.send({ type: 'SetUsername', username: name });
        this.messageInput.disabled = false;
        this.messageForm.querySelector('button').disabled = false;
    }

    joinRoom(room) {
        this.currentRoom = room;
        this.currentRoomDisplay.textContent = `# ${room}`;
        this.send({ type: 'JoinRoom', room });
        this.messagesDiv.innerHTML = '';
        
        document.querySelectorAll('.room-item').forEach(item => {
            item.classList.remove('active');
        });
        document.querySelector(`[data-room="${room}"]`)?.classList.add('active');
    }

    sendMessage(e) {
        e.preventDefault();
        
        const content = this.messageInput.value.trim();
        if (!content) return;

        if (content.startsWith('/')) {
            this.handleCommand(content);
        } else {
            if (!this.currentRoom) {
                this.addSystemMessage('Please join a room first');
                return;
            }
            this.send({ type: 'SendMessage', content });
        }

        this.messageInput.value = '';
    }

    handleCommand(content) {
        const parts = content.split(' ');
        const command = parts[0];

        switch (command) {
            case '/nick':
                if (parts[1]) {
                    this.setUsername(parts[1]);
                }
                break;
            case '/join':
                if (parts[1]) {
                    this.joinRoom(parts[1]);
                    this.addRoomToList(parts[1]);
                }
                break;
            case '/pm':
                if (parts.length >= 3) {
                    const to = parts[1];
                    const message = parts.slice(2).join(' ');
                    this.send({ type: 'PrivateMessage', to, content: message });
                }
                break;
            case '/rooms':
                this.send({ type: 'ListRooms' });
                break;
            default:
                this.addSystemMessage('Unknown command');
        }
    }

    send(data) {
        if (this.ws && this.connected) {
            this.ws.send(JSON.stringify(data));
        }
    }

    addMessage(username, content, timestamp) {
        const messageEl = document.createElement('div');
        messageEl.className = 'message';
        messageEl.innerHTML = `
            <div class="message-header">
                <span class="message-username">${this.escapeHtml(username)}</span>
                <span class="message-time">${this.formatTime(timestamp)}</span>
            </div>
            <div class="message-content">${this.escapeHtml(content)}</div>
        `;
        this.messagesDiv.appendChild(messageEl);
        this.scrollToBottom();
    }

    addPrivateMessage(from, content, timestamp) {
        const messageEl = document.createElement('div');
        messageEl.className = 'message private';
        messageEl.innerHTML = `
            <div class="message-header">
                <span class="message-username">🔒 ${this.escapeHtml(from)} (private)</span>
                <span class="message-time">${this.formatTime(timestamp)}</span>
            </div>
            <div class="message-content">${this.escapeHtml(content)}</div>
        `;
        this.messagesDiv.appendChild(messageEl);
        this.scrollToBottom();
    }

    addSystemMessage(content) {
        const messageEl = document.createElement('div');
        messageEl.className = 'message system';
        messageEl.innerHTML = `
            <div class="message-header">
                <span class="message-username">System</span>
                <span class="message-time">${this.formatTime(new Date().toISOString())}</span>
            </div>
            <div class="message-content">${this.escapeHtml(content)}</div>
        `;
        this.messagesDiv.appendChild(messageEl);
        this.scrollToBottom();
    }

    addRoomToList(room) {
        if (document.querySelector(`[data-room="${room}"]`)) return;

        const roomEl = document.createElement('div');
        roomEl.className = 'room-item';
        roomEl.dataset.room = room;
        roomEl.innerHTML = `
            <span># ${room}</span>
        `;
        roomEl.addEventListener('click', () => this.joinRoom(room));
        this.roomsList.appendChild(roomEl);
    }

    formatTime(timestamp) {
        const date = new Date(timestamp);
        return date.toLocaleTimeString('en-US', { 
            hour: '2-digit', 
            minute: '2-digit' 
        });
    }

    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }

    scrollToBottom() {
        this.messagesDiv.scrollTop = this.messagesDiv.scrollHeight;
    }
}

// Initialize the chat client
const chat = new ChatClient();
