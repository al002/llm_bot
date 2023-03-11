## Get started

### Configuration

```
TELEGRAM_BOT_TOKEN="YOUR_TELEGRAM_BOT_TOKEN"
OPENAI_API_KEY="YOUR_OPENAI_API_KEY"
DB_REDIS="YOUR_REDIS_URL"
```

copy `.env.example` to `.env` then change configuration.

### Run
```shell
git clone https://github.com/al002/llm_bot.git

cd llm_bot

cargo run
```

### Use custom LLM
Just connect to [https://github.com/al002/programming_llm](https://github.com/al002/programming_llm) or any other gRPC server that provide same functionality.
