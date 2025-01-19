---
icon: sitemap
---

# System Architecture

```mermaid fullWidth="false"

flowchart LR
    A([User<br/>Web / Mobile]) --> B(RavenMarket Frontend DApp)
    B --> |Create/participate prediction events, chat interaction, data submission| C(Raven AI Engine)
    B --> |Create/participate prediction events, submit transactions| F(Smart Contracts<br/>RM Token)
    C --> |Invoke ChatGPT / GPT-Like Models| D(ChatGPT / LLM)
    C --> |Real-time data queries| E(Pyth Oracle / Other Data Sources)
    F --> |Execute trading logic / settlement<br/>Record on-chain| G(Blockchain Network)
    C --> |Analysis results / predictions| B
    G --> |Transaction info, prediction outcomes| B
    B --> |Text / Voice / IM / RTC| H(Real-time Interaction<br/>Chat / Voice / Video)

    classDef default fill:#f9f,stroke:#333,stroke-width:1px,color:#000
    classDef data fill:#BBF3FF,stroke:#333,stroke-width:1px,color:#000
    classDef compute fill:#BFFFC8,stroke:#333,stroke-width:1px,color:#000
    classDef contract fill:#FFF8B2,stroke:#333,stroke-width:1px,color:#000
    classDef user fill:#FFDCFF,stroke:#333,stroke-width:1px,color:#000
    classDef external fill:#FFEEB2,stroke:#333,stroke-width:1px,color:#000

    class A user
    class B default
    class C compute
    class D external
    class E data
    class F contract
    class G contract
    class H default
```



## The Three-Eyed Raven: The Symbol of the Decentralized Era

The three-eyed raven transcends the ordinary, representing mastery over the past, present, and future. As a bridge between knowledge and the unknown, it merges ancient wisdom with modern technology, offering unmatched precision and foresight.



```mermaid fullWidth="false"
flowchart LR
    %% --- Three-Eyed Raven as the central entity ---
    subgraph Raven[Three-Eyed Raven<br/>Bridging Knowledge & the Unknown]
        direction TB
        
        %% First Eye: Witnessing the Past
        subgraph Past[Witnessing the Past]
        P1[Crypto cycles & market history]
        P2[Lessons from bull & bear experiences]
        end
        
        %% Second Eye: Understanding the Present
        subgraph Present[Understanding the Present]
        S1[Real-time prediction markets]
        S2[Sentiment analysis<br/>Trend identification]
        end
        
        %% Third Eye: Envisioning the Future
        subgraph Future[Envisioning the Future]
        F1[Powered by Raven AI<br/>Big data analytics]
        F2[Expert-level predictions]
        F3[Decision-making support]
        end
    end
    
    %% --- External or overarching elements ---
    KnowledgeBase[Ancient Wisdom +<br/>Modern Technology]
    Ecosystem[Decentralized Global Ecosystem]

    %% --- Connections ---
    KnowledgeBase --> Raven
    Raven --> Ecosystem

```

### **The First Eye – Witnessing the Past**

Drawing lessons from the crypto world’s cycles of bull and bear markets and the countless experiences that shaped the industry.

### **The Second Eye – Understanding the Present**

Deeply integrated with real-time prediction markets, it uncovers market sentiments and identifies developing trends.

### **The Third Eye – Envisioning the Future**

Powered by Raven AI, the platform analyzes vast datasets and user behavior to provide expert-level predictions and decision-making support.

## **Mission: Predictions Beyond Simple Bets**

In _From Prediction Markets to Information Finance_, Vitalik envisions a future where "prediction" is no longer just a simple vote or bet, but a fundamental element in how the world makes decisions, drives innovation, governs, and evolves.

```mermaid
flowchart LR
    A([Users & External Data<br/>Inputs from Communities, Oracles, etc.]) --> B(Prediction / Info-Finance Hub)
    B --> C(Decision-Making)
    B --> D(Innovation)
    B --> E(Governance)
    B --> F(Evolution)
    
    classDef nodeStyle fill:#FFF8B2,stroke:#333,stroke-width:1px,color:#000
    classDef dataStyle fill:#FFEEB2,stroke:#333,stroke-width:1px,color:#000
    
    class A dataStyle
    class B nodeStyle
    class C,D,E,F nodeStyle

```

RavenMarket is the key to unlocking this future:

* **Predict to Earn:** With AI-driven models, users can easily create prediction events without technical expertise. Early participants and liquidity providers are rewarded, sharing in the platform’s growth.
* **Pyth Oracle:** Ensuring real-time, authoritative data accuracy, RavenMarket is the only prediction platform fully powered by Pyth Oracle, offering unparalleled precision and reliability.
* **Raven AI:** Advanced AI algorithms mine data, track trends, and assist in decision-making. Whether you're a tech expert or a novice, you’ll have your own AI empowered Three-Eyed Raven, offering personalized insights.
* **Real-Time Social Interaction:** Through text, voice, IM, and RTC, users engage in dynamic discussions. Raven AI records and learns from trading habits and effective strategies, creating personalized data samples for better decision support.
* **Smart Contracts:** Transparent, on-chain transactions ensure every trade is publicly verifiable — no hidden agendas, just complete trust.
* **Value Sharing:** 50% of platform profits are allocated to buy back RM tokens, allowing every participant to become a co-creator of the ecosystem.

Unlike traditional "guess the trend" platforms, RavenMarket transforms prediction into a global "consensus experiment," blending collective intuition, experience, and AI algorithms to produce highly accurate outcomes. It allows information to evolve within a decentralized framework, fostering continuous innovation.

## **Scenarios: From Native to Expansive**

RavenMarket isn’t just a tool—it’s a revolutionary philosophy, bringing multidimensional value through market practice.

```mermaid
flowchart LR
    %% Central Node
    RM([RavenMarket<br/>A Revolutionary Philosophy])
    
    %% Scenario: Hot Assets Trading
    subgraph S1[Hot Assets Trading]
      S1A[Trending tokens, global hotspots]
      S1B[Efficient market makers<br/>optimal entries/exits]
    end
    
    %% Scenario: Classic Continuity
    subgraph S2[Classic Continuity]
      S2A[Recognized IPs: Pepe, Cheems]
      S2B[Community-driven resonance<br/>transforms classics]
    end
    
    %% Scenario: Original Narrative
    subgraph S3[Original Narrative]
      S3A[New ideas: NFTs, GameFi, RWA...]
      S3B[Flexible creation of<br/>entirely new projects]
    end
    
    %% Scenario: Evolving Expansion
    subgraph S4[Evolving Expansion]
      S4A[AI + DeSci, AI bots/agents]
      S4B[Community-built<br/>decentralized markets]
    end
    
    %% Connections
    RM --> S1
    RM --> S2
    RM --> S3
    RM --> S4

    %% (Optional) Styling
    classDef mainNode fill:#FFF8B2,stroke:#333,stroke-width:1px,color:#000
    classDef scenario fill:#FFEEB2,stroke:#333,stroke-width:1px,color:#000
    
    class RM mainNode
    class S1,S2,S3,S4 scenario

```

**Hot Assets Trading**

1. RavenMarket thrives on trending events, such as predictions on hot tokens' prices or global hotspots' outcomes.
2. Market makers efficiently capitalize on trends, entering and exiting at optimal moments.

**Classic Continuity**

1. Globally recognized IPs, such as Pepe or Cheems, find new life as prediction assets on RavenMarket.
2. Community-driven resonance ensures liquidity while transforming classics into innovative market experiences.

**Original Narrative**

1. Great autonomy and flexibility allow users to integrate new ideas and innovative concepts, such as NFTs, GameFi, RWA, and more, to create entirely new prediction projects.
2. With its endless supply of original content and creative possibilities, RavenMarket holds infinite potential.

**Evolving Expansion**

1. As AI continues to advance, RavenMarket is primed to incubate and develop more AI-driven projects, such as AI + DeSci, AI bots, AI agents and beyond.
2. Community-inspired projects can leverage RavenMarket’s infrastructure to build their own decentralized prediction markets.

\
RavenMarket’s strength lies in its continuous evolution. Unlike platforms confined to a single trend, RavenMarket constantly adapts and grows, incorporating new prediction scenarios and enhancing user interactions. This dynamic expansion ensures that RM tokens retain long-term value, while empowering users to explore an ever-expanding digital universe.

With RavenMarket, the future isn’t just imagined—it’s predicted, shared, and shaped by everyone.

## Raven AI Overview



```mermaid
flowchart LR
    %% Subgraph: User Interaction Layer
    subgraph A[User Interaction Layer]
    A1([DApp UI<br/>Web/Mobile])
    A2([IM/Chat/Voice/RTC])
    end
    
    %% Subgraph: Data Sources
    subgraph B[Data Sources]
    B1[On-chain Data - transactions]
    B2[Pyth Oracle - Market Prices]
    B3[External APIs & Feeds - News, Social Media]
    end

    %% Subgraph: Data Ingestion & Processing
    subgraph C[Data Ingestion & Processing]
    C1[Data Aggregator]
    C2[Data Transformation<br/>Normalization, Cleaning]
    C3[Data Storage<br/>Database / Data Lake]
    end

    %% Subgraph: Raven AI - Core
    subgraph D[Raven AI - Core]
    D1[Raven AI Orchestrator]
    D2[Prediction Models<br/>ML & DL]
    D3[Behavioral Analysis<br/>User Patterns, Sentiment]
    D4[Knowledge Base<br/>Aggregated Data]
    end

    %% Subgraph: LLM & Inference Layer
    subgraph E[LLM & Inference Layer]
    E1[ChatGPT / GPT-like Model]
    E2[Prompt Engineering<br/>Context Builder]
    E3[Inference Engine<br/>API Gateway]
    end

    %% Subgraph: Feedback & Continuous Learning
    subgraph F[Feedback & Continuous Learning]
    F1[User Feedback<br/>Ratings, Votes, Comments]
    F2[Model Fine-tuning<br/>Reinforcement Learning]
    F3[Behavior Logging<br/>Clickstream, Results]
    end

    %% Flows between components
    A1 --> |User requests / queries / new prediction events| D1
    A2 --> |User chats / voice / social interaction| D1

    B1 --> |On-chain data| C1
    B2 --> |Real-time price and event data| C1
    B3 --> |News / social media / external indicators| C1

    C1 --> |Data ingestion| C2
    C2 --> |Cleaned / normalized data| C3
    C3 --> |Enriched dataset| D4

    D1 --> |Sub-tasks, data context| D2
    D1 --> |Analyze user behavior| D3
    D2 --> |Market predictions<br/>Probability outcomes| E3
    D3 --> |Behavior insights<br/>Decision patterns| E2
    D4 --> |Historical and contextual data| E2

    E2 --> |Construct prompts and context| E1
    E1 --> |Generated predictions, insights, strategies| E3
    E3 --> |LLM-based recommendations and output| D1

    D1 --> |AI-driven insights<br/>Strategies, analysis| A1
    D1 --> |Social / voice analysis| A2

    A1 --> |User results, performance data| F1
    A2 --> |User feedback, engagement| F1
    F1 --> |Improve user behavior models| D3
    F1 --> |Retrain or fine-tune models| F2
    F2 --> |Update ML / AI models| D2
    F2 --> |Refine AI orchestrator logic| D1
    F2 --> |Enrich knowledge base| D4
    F1 --> |Logs & usage metrics| C3

    %% Optional styling
    style A1 fill:#FFDCFF,stroke:#333,color:#000
    style A2 fill:#FFDCFF,stroke:#333,color:#000
    style B1 fill:#BBF3FF,stroke:#333,color:#000
    style B2 fill:#BBF3FF,stroke:#333,color:#000
    style B3 fill:#BBF3FF,stroke:#333,color:#000
    style C1 fill:#BFFFC8,stroke:#333,color:#000
    style C2 fill:#BFFFC8,stroke:#333,color:#000
    style C3 fill:#BFFFC8,stroke:#333,color:#000
    style D1 fill:#FFF8B2,stroke:#333,color:#000
    style D2 fill:#FFF8B2,stroke:#333,color:#000
    style D3 fill:#FFF8B2,stroke:#333,color:#000
    style D4 fill:#FFF8B2,stroke:#333,color:#000
    style E1 fill:#FFEEB2,stroke:#333,color:#000
    style E2 fill:#FFEEB2,stroke:#333,color:#000
    style E3 fill:#FFEEB2,stroke:#333,color:#000
    style F1 fill:#f9f,stroke:#333,color:#000
    style F2 fill:#f9f,stroke:#333,color:#000
    style F3 fill:#f9f,stroke:#333,color:#000
```

## Raven Prediction System



```mermaid
flowchart LR

    subgraph U[User Side]
    U1[User Wallet - Anchor Client]
    U2[AI-Assisted Frontend]
    end

    subgraph SC[anchor_prediction_market Program - on chain]
        direction TB
        
        subgraph Instr[Instruction Handlers]
        I1[init_state]
        I2[add_price_feed, remove_price_feed]
        I3[create_market, pause_market, resume_market]
        I4[user_bet]
        I5[auto_settle_all]
        I6[update_settle_incentive]
        I7[query_*]
        end
        
        subgraph DataAcc[Contract Accounts - PDAs]
        DA1[State Account]
        DA2[Escrow Vault PDA]
        end

        subgraph MarketStructs[Markets and Rounds]
        MS1[Market - market_id, config, rounds]
        MS2[Round - bets, settled...]
        MS3[Bet - user, amount, direction]
        end
    end

    subgraph AI[AI Subsystem - Off chain or Hybrid]
    AI1[AI Engine - ChatGPT or LLM]
    AI2[Data Analyzer - On chain data, user behavior]
    end

    subgraph SP[System & External]
    SP1[System Program - SOL transfers]
    SP2[Pyth Oracle - Price feeds]
    end

    U1 -->|User consults AI services| U2
    U2 -->|Requests strategy or analysis| AI1
    AI1 -->|Fetch on chain data if needed| AI2
    AI2 -->|Aggregated info, predictions| AI1
    AI1 -->|Returns suggestions or insights| U2

    U1 -->|Calls instructions| I1
    U1 -->|Calls instructions| I2
    U1 -->|Calls instructions| I3
    U1 -->|Calls instructions| I4
    U1 -->|Calls instructions| I5
    U1 -->|Calls instructions| I6
    U1 -->|Calls instructions| I7

    %% --- Instruction to PDAs
    I1 --> DA1
    I1 --> DA2
    I2 --> DA1
    I3 --> DA1
    I3 --> MS1
    I4 --> DA1
    I4 --> DA2
    I4 --> MS1
    I4 --> MS2
    I4 --> MS3
    I5 --> DA1
    I5 --> DA2
    I5 --> MS1
    I5 --> MS2
    I5 --> MS3
    I6 --> DA1
    I7 --> DA1
    I7 --> MS1
    I7 --> MS2
    I7 --> MS3

    I3 -->|Creation fee| SP1
    I4 -->|User bet transfer| SP1
    I5 -->|Fetch price data| SP2
    I5 -->|Escrow payouts| SP1

    classDef highlight fill:#FFF8B2,stroke:#333,stroke-width:1px,color:#000
    classDef account fill:#BBF3FF,stroke:#333,stroke-width:1px,color:#000
    classDef external fill:#FFEEB2,stroke:#333,stroke-width:1px,color:#000
    classDef user fill:#FFDCFF,stroke:#333,stroke-width:1px,color:#000
    classDef ai fill:#FFCFCF,stroke:#333,stroke-width:1px,color:#000
    
    class U1,U2 user
    class AI1,AI2 ai
    class I1,I2,I3,I4,I5,I6,I7 highlight
    class DA1,DA2,MS1,MS2,MS3 account
    class SP1,SP2 external

```

