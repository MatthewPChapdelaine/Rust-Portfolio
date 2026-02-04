// Main JavaScript file
console.log('Blog Engine loaded');

// Utility functions
function formatDate(dateString) {
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'long',
        day: 'numeric'
    });
}

// Auth helpers
function setAuthToken(token) {
    localStorage.setItem('auth_token', token);
}

function getAuthToken() {
    return localStorage.getItem('auth_token');
}

function removeAuthToken() {
    localStorage.removeItem('auth_token');
}

// API helpers
async function apiRequest(url, options = {}) {
    const token = getAuthToken();
    const headers = {
        'Content-Type': 'application/json',
        ...options.headers
    };

    if (token) {
        headers['Authorization'] = `Bearer ${token}`;
    }

    const response = await fetch(url, {
        ...options,
        headers
    });

    return response;
}
