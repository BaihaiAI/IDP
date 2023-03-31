# Awesome Foundational LLMs

A list of foundational LLM (Large Language Model) projects and the important news of LLMs.

# Foundational LLMs

## 1. **ChatGLM-6B**

ChatGLM-6B is a bilingual language model, which can be run on a single consumer-grade GPU.

The model is based on the GLM architecture and has 6.2 billion parameters.

ChatGLM-6B is trained on 1T identifiers in both Chinese and English, supported by supervised fine-tuning, feedback self-help, and human feedback reinforcement learning. Thus, though ChatGLM-6B is not as large as the 100 billion model, it has greatly reduced the inference cost, improved the efficiency, and has been able to generate responses that are quite consistent with human preferences. 

Project repo: [https://github.com/THUDM/ChatGLM-6B](https://github.com/THUDM/ChatGLM-6B)


## 2. **OpenChatKit**

**OpenChatKit is known as the  “Open source ChatGPT alternative”.**

OpenChatkit is an open-source base to create both specialized and general purpose chatbots, supporting both English and Chinese. The kit includes an instruction-tuned 20 billion parameter language model, a 6 billion parameter moderation model, and an extensible retrieval system for including up-to-date responses from custom repositories.

Currently, the model do good at multiple tasks, including summary, question answering with context, message extraction, text classification, etc.

But it's not quite as good at creative writing, coding, multi-rounds conversations and being sluggish when switching topics ...

The model is develeoped by Togegher, a startup founded in 2022/07. The company is aiming at building a decentralized cloud for AI.

**Project repo:** [https://github.com/togethercomputer/OpenChatKit](https://github.com/togethercomputer/OpenChatKit)

**Blog**: [https://www.together.xyz/blog/openchatkit](https://www.together.xyz/blog/openchatkit)


## 3. LLaMA

LLaMA is a foundational Large Language Model released by Meta. It is featured for “smaller while performant”, enabling users who don’t have access to large amounts of infrastructure to study LLMs.

LLaMA is available at 4 sizes：7B, 13B, 33B, and 65B parameters.

LLaMA Inference Repository: [https://github.com/facebookresearch/llama](https://github.com/facebookresearch/llama)

LLaMA quick start with Python: [https://github.com/BaihaiAI/IDP/blob/main/AIGC/LLaMA.ipynb](https://github.com/BaihaiAI/IDP/blob/main/AIGC/LLaMA.ipynb)

### 1) **ChatLLaMA**

Three days after the launch of LLaMA, startup AI company Nebuly AI has built ChatLLaMA.

ChatLLaMA is a library that allows you to create hyper-personalized ChatGPT-like assistants using your own data and the least amount of compute possible.

**Project Repo**: [https://github.com/nebuly-ai/nebullvm](https://github.com/nebuly-ai/nebullvm)


### **2) Standford Alpaca**

Alpaca is an open source instruction-following language model, fine-tuned from LLaMA. According to Standford HAI, “*Alpaca 7B behaves similarly to OpenAI’s text-davinci-003, while being surprisingly small and easy/cheap to reproduce. ”*

Project repo: [https://github.com/tatsu-lab/stanford_alpaca](https://github.com/tatsu-lab/stanford_alpaca)

Trial: [https://alpaca-ai-custom5.ngrok.io/](https://alpaca-ai-custom5.ngrok.io/)

Note: Sinece LLaMA is not open sourced yet, LLaMA and LLaMA-based models are currently prohibited from commercial use.

## 4. Databricks Dolly

Dolly-v1-6b is a 6 billion parameter causal language model created by Databricks that is derived from EleutherAI’s GPT-J (released June 2021) and fine-tuned on a ~52K record instruction corpus (Stanford Alpaca) consisting of question/answer pairs generated using the techniques outlined in the Self-Instruct paper. Dolly was trained using deepspeed ZeRO 3 on the Databricks Machine Learning Platform in just 30 minutes using a single NDasrA100_v4 machine with 8x A100 40GB GPUs.

Project repo: [https://github.com/databrickslabs/dolly](https://github.com/databrickslabs/dolly)

Contributions on the list and content are well welcome!
