# Description
This Rust project fetches and displays the current weather for a specified city using the OpenWeatherMap API. It demonstrates the use of Actix-web for handling HTTP requests and responses, as well as integration with the reqwest crate for making external API calls.
The project also includes error handling and environment variable management using the dotenv crate.

# Features
- Fetches real-time weather data for a specified city.
- Displays temperature and city name.
- Handles errors and edge cases gracefully.
- Uses environment variables to manage API keys securely.

# Setup Instructions

1. Clone the repository:
```
git clone https://github.com/your-username/Weather-App-in-Rust.git
cd Weather-App-in-Rust
```

2. Create a .env file in the root of the project with your OpenWeatherMap API key:

```
OPENWEATHERMAP_API_KEY=your_api_key

```

3. Run the project:
```
cargo run
```

# Usage
- The server will start at 127.0.0.1:8080.
- Access the weather information by navigating to http://127.0.0.1:8080 in your browser or using curl.

# Dependencies
- actix-web: Web framework for Rust.
- reqwest: HTTP client for making requests.
- dotenv: Loads environment variables from a .env file.
