# Карта знаний: от основ ML до работы в исследовательской AI-лаборатории

Цель — стать специалистом, который понимает модели, умеет надёжно их обучать, исследовать и улучшать, а результаты подтверждает экспериментами и работающими системами. Ориентир — технические роли в лабораториях уровня OpenAI и Anthropic.

Это широкая карта тем, а не официальный список требований компаний и не гарантия найма. Область развивается, поэтому законченного списка «всего ML навсегда» не существует. Нужны крепкое общее ядро, глубокая специализация и доказательства самостоятельной работы. Не требуется быть экспертом во всех перечисленных направлениях одновременно.

## 1. Как пользоваться картой

**Приоритеты:** **A** — обязательная база; **B** — общее ядро для работы с современными глубокими моделями; **C** — углубление по выбранной специализации. В смешанных разделах глубина указана отдельно.

Галочка у темы означает, что ты можешь объяснить её своими словами, решить небольшую задачу, реализовать ключевую идею или проверить реализацию и назвать ограничения. Просмотра лекции недостаточно. Для сложных систем не обязательно писать всё с нуля: нужно понимать устройство, измерять поведение и диагностировать ошибки.

Для каждого блока сохраняй: конспект, код, проверку корректности, результаты эксперимента и объяснение неудачных попыток. Полезный шаблон записи: `тема → гипотеза → реализация → проверка → результат → ограничения → следующий вопрос`.

В этом репозитории уже есть [пошаговые уроки на Rust](LEARNING_PLAN.md), [математические основы](FOUNDATIONS.md), [самопроверка](SELF_CHECK.md), [прогресс](PROGRESS.md) и [датасеты](DATASETS.md). Используй их для освоения базы. Наличие урока или готового кода само по себе не доказывает владение темой. Параллельно освой Python и PyTorch для исследовательской практики.

## 2. Выбор профессиональной траектории

1. **Research Engineer**
   - Главная глубина: Реализация идей, обучение, отладка, эксперименты, производительность.
   - Чем подтверждать уровень: Воспроизведение статьи, устойчивый training pipeline, измеримое улучшение.

2. **Research Scientist**
   - Главная глубина: Постановка вопросов, новые методы, теория и эксперимент.
   - Чем подтверждать уровень: Самостоятельное исследование, убедительные сравнения, статьи или сильные исследовательские проекты.

3. **ML Systems / Infrastructure Engineer**
   - Главная глубина: GPU, распределённое обучение, serving, надёжность.
   - Чем подтверждать уровень: Профилирование, ускорение, масштабирование, восстановление после сбоев.

4. **Applied ML Engineer**
   - Главная глубина: Данные, качество в реальной задаче, deployment, продуктовые ограничения.
   - Чем подтверждать уровень: Работающая система с измеримым качеством, стоимостью и мониторингом.

5. **Alignment / Safety Researcher or Engineer**
   - Главная глубина: Поведение моделей, оценивание, интерпретируемость, устойчивость.
   - Чем подтверждать уровень: Воспроизводимые исследования рисков и проверка эффективности методов.


Границы ролей пересекаются. Выбери одну основную и одну соседнюю специализацию после общего ядра.

В официальном описании [Research Engineer в OpenAI](https://openai.com/careers/research-engineer-san-francisco/) выделяются сильное программирование, распределённые системы и эффективная реализация глубокого обучения. Для [Research Scientist](https://openai.com/careers/research-scientist-san-francisco/) важны новые идеи, результаты исследований и самостоятельное ведение исследовательской программы. [Anthropic](https://www.anthropic.com/careers) описывает эмпирический подход, совместную работу и создание надёжных, понятных и безопасных AI-систем. Это ориентиры при составлении карты; конкретную подготовку сверяй с выбранной вакансией. Страницы проверены 27 октября 2026 года.

## 3. Математический язык и линейная алгебра — A

1. [ ] Функции, множества, логика, суммы и произведения, степени и логарифмы, индексы и математические обозначения.
2. [ ] Доказательства: прямое рассуждение, контрпример, индукция; различие утверждения, предположения и эмпирического наблюдения.
3. [ ] Векторы, скалярное произведение, нормы, расстояния, углы, проекции и ортогональность.
4. [ ] Матрицы как линейные преобразования; размерности, транспонирование, произведение и след.
5. [ ] Линейная независимость, базис, ранг, образ, ядро и системы линейных уравнений.
6. [ ] Собственные значения и векторы, симметричные и положительно определённые матрицы.
7. [ ] SVD, QR, псевдообратная матрица, метод наименьших квадратов и низкоранговые приближения.
8. [ ] Тензоры, свёртки индексов, broadcasting, batch dimensions, `einsum`.
9. [ ] Число обусловленности и влияние геометрии данных на обучение.

**Проверка:** объяснить PCA через SVD; вывести решение линейной регрессии и объяснить, почему численно обычно решают систему, а не явно вычисляют обратную матрицу.

## 4. Математический анализ и оптимизация — A, углубление C

1. [ ] Предел, непрерывность, производная, частные производные, интеграл и математическое ожидание как интеграл.
2. [ ] Правило цепочки, градиент, Якобиан, Гессиан, производная по направлению.
3. [ ] Матричное дифференцирование, vector-Jacobian и Jacobian-vector products.
4. [ ] Ряд Тейлора, локальные приближения, гладкость и кривизна.
5. [ ] Выпуклость, локальные и глобальные минимумы, седловые точки.
6. [ ] Gradient descent, SGD, momentum, Adam/AdamW; различие L2-регуляризации и decoupled weight decay.
7. [ ] Learning rate, warmup, schedules, batch size, gradient accumulation, clipping.
8. [ ] Ограниченная оптимизация, множители Лагранжа, условия KKT и двойственность — обзор; глубоко для теоретической специализации.
9. [ ] Методы второго порядка, natural gradient, приближённая кривизна, стохастические оценки градиента — C.
10. [ ] Reparameterization trick и score-function estimator; смещение и дисперсия оценок — B/C.

**Проверка:** вручную вывести backpropagation для двухслойной сети, проверить градиенты конечными разностями и объяснить расходимость при слишком большом шаге.

## 5. Вероятность, статистика и теория информации — A

1. [ ] Условная вероятность, независимость, формула Байеса и формула полной вероятности.
2. [ ] Случайные величины, плотность, распределение, математическое ожидание, дисперсия, ковариация и корреляция.
3. [ ] Bernoulli, categorical, binomial, Gaussian, Poisson, exponential, beta и Dirichlet distributions.
4. [ ] Совместные и условные распределения, маргинализация, условное ожидание.
5. [ ] Закон больших чисел, центральная предельная теорема, Monte Carlo.
6. [ ] Likelihood, log-likelihood, MLE, MAP, prior и posterior, posterior predictive.
7. [ ] Смещение и дисперсия оценок, доверительные интервалы, bootstrap и permutation tests.
8. [ ] Проверка гипотез, мощность, размер эффекта, множественные сравнения и ошибки последовательного подглядывания в результаты.
9. [ ] Дизайн A/B-экспериментов; единица рандомизации, зависимость наблюдений и размер выборки.
10. [ ] Энтропия, cross-entropy, KL divergence, mutual information; почему KL не является метрикой расстояния.
11. [ ] Отличие корреляции от причинности; confounding, selection bias, causal graphs и вмешательства.
12. [ ] Importance sampling, MCMC, variational inference, вероятностные графические модели — углубление C.

**Проверка:** сравнить две модели с учётом парности примеров и неопределённости; объяснить, почему небольшой прирост на одном запуске ещё не доказывает улучшение.

## 6. Численные вычисления — A/B

1. [ ] Floating point: FP64, FP32, FP16, BF16; диапазон, точность, округление, overflow и underflow.
2. [ ] Катастрофическое сокращение, неассоциативность сложения, накопление ошибки, устойчивые редукции.
3. [ ] Численно устойчивые sigmoid, softmax, log-softmax, log-sum-exp и cross-entropy.
4. [ ] Масштабирование признаков, инициализация, нормализация и контроль величин активаций.
5. [ ] Mixed precision, loss scaling, master weights; различия требований FP16 и BF16.
6. [ ] Воспроизводимость, seeds, недетерминированные операции и пределы побитовой идентичности.
7. [ ] Сравнение реализаций с абсолютным и относительным допуском.

**Проверка:** воспроизвести NaN/Inf в нестабильном softmax, устранить причину и сравнить точность нескольких dtype.

## 7. Программирование и компьютерные системы — A

1. [ ] Python: структуры данных, функции, классы, итераторы, генераторы, контекстные менеджеры, typing, исключения и управление зависимостями.
2. [ ] NumPy: векторизация, индексирование, views/copies, strides, broadcasting и генерация случайных чисел.
3. [ ] Алгоритмы и структуры данных: массивы, hash tables, деревья, графы, heaps, сортировка, поиск и сложность по времени/памяти.
4. [ ] Git, Linux, shell, процессы, потоки, права доступа, переменные окружения и удалённая работа.
5. [ ] Архитектура компьютера: CPU, RAM, cache, SIMD, I/O; latency, bandwidth и locality.
6. [ ] Конкурентность, multiprocessing, async I/O, synchronization, race conditions и deadlocks.
7. [ ] Сети: TCP, HTTP, RPC, retries, timeouts, backpressure и идемпотентность.
8. [ ] SQL: joins, aggregations, window functions; схемы и индексы.
9. [ ] Тестирование: unit, integration, property-based, regression; debugging и profiling.
10. [ ] Структура пакетов, конфиги, логирование, CLI, CI, code review и документация.
11. [ ] C++ и CUDA для системной специализации; Rust полезен для надёжных компонентов и изучения устройства вычислений.

**Проверка:** превратить notebook в воспроизводимый пакет с CLI, конфигом, тестами и понятной обработкой ошибок.

## 8. Данные и постановка задачи — A

1. [ ] Перевод практической задачи в целевую переменную, функцию потерь, метрику и ограничения.
2. [ ] Сбор, происхождение, лицензии, согласие на использование и приватность данных.
3. [ ] Разметка: инструкции, качество, согласие аннотаторов, неоднозначность и шум.
4. [ ] EDA, пропуски, выбросы, дубликаты, дисбаланс, ошибки единиц измерения и схем.
5. [ ] Train/validation/test; group split, temporal split, stratification и cross-validation.
6. [ ] Leakage: статистики preprocessing, связанные объекты, дубликаты и будущая информация.
7. [ ] Feature engineering, числовые и категориальные признаки, нормализация и кодирование категорий.
8. [ ] Dataset shift: covariate shift, label shift, concept drift; OOD-данные.
9. [ ] Версионирование, lineage, manifests, hashes, воспроизводимые преобразования и data contracts.
10. [ ] Streaming, sharding, columnar formats, data loaders, prefetching и ограничения I/O.
11. [ ] Active learning, weak supervision, synthetic data и semi-supervised learning — обзор, углубление по задаче.

**Проверка:** построить pipeline, в котором все обучаемые преобразования используют только train, и найти намеренно внесённую утечку.

## 9. Классическое машинное обучение — A

1. [ ] Supervised, unsupervised, self-supervised и reinforcement learning; generative и discriminative models.
2. [ ] Линейная и логистическая регрессия; ridge, lasso, elastic net, GLM.
3. [ ] k-NN, naive Bayes, decision trees, random forests, gradient boosting.
4. [ ] SVM, kernels и Gaussian processes: основные идеи и условия применимости.
5. [ ] Clustering: k-means, hierarchical clustering, DBSCAN, Gaussian mixtures и EM.
6. [ ] PCA, matrix factorization; ограничения t-SNE/UMAP как визуализации.
7. [ ] Bias–variance tradeoff, overfitting, underfitting, regularization и learning curves.
8. [ ] Hyperparameter search: random/grid search, Bayesian optimization и честный бюджет сравнения.
9. [ ] MAE/MSE, precision/recall/F1, ROC-AUC/PR-AUC, log-loss, calibration и выбор порога по стоимости ошибок.
10. [ ] Ranking/recommendations: retrieval, NDCG, recall@k, collaborative filtering, implicit feedback и exposure bias.
11. [ ] Временные ряды: лаги, сезонность, rolling validation, forecasting и временная утечка.
12. [ ] Аномалии, cost-sensitive learning, uncertainty estimation и conformal prediction с условиями применимости.

**Проверка:** решить табличную задачу с простым baseline и бустингом, объяснить выбор метрики и показать ошибки по подгруппам.

## 10. Теория обучения — B, доказательства C

1. [ ] Empirical risk minimization, population risk и generalization gap.
2. [ ] IID-предположение и последствия его нарушения.
3. [ ] Capacity, inductive bias, sample complexity, VC dimension и Rademacher complexity.
4. [ ] Регуляризация как ограничение класса решений; early stopping и implicit bias оптимизатора.
5. [ ] Double descent, overparameterization, interpolation и ограничения классических объяснений.
6. [ ] Distribution shift, robustness, invariance и domain adaptation.
7. [ ] Различие теоремы при заданных предположениях и наблюдения на конкретном benchmark.

**Проверка:** назвать предположения выбранной оценки обобщения и объяснить, что она позволяет заключить о реальной нейросети.

## 11. Deep learning и PyTorch — B

1. [ ] Perceptron, MLP, activations, loss functions, embeddings и parameter sharing.
2. [ ] Computational graph, reverse-mode autodiff, leaf tensors, detach, in-place operations и gradient accumulation.
3. [ ] Инициализация Xavier/He, vanishing/exploding gradients, residual connections.
4. [ ] BatchNorm, LayerNorm, RMSNorm; различие train/eval и статистик нормализации.
5. [ ] Dropout, augmentation, label smoothing, weight decay и early stopping.
6. [ ] PyTorch: tensors, modules, parameters, buffers, optimizers, datasets и dataloaders.
7. [ ] Training loop, validation loop, masking, reduction losses и корректный подсчёт метрик.
8. [ ] Checkpoints: веса, optimizer, scheduler, RNG и положение в потоке данных.
9. [ ] Hooks, gradient checking, anomaly detection, profiler и диагностика памяти.
10. [ ] Compilation и graph breaks; eager execution и скомпилированные графы.
11. [ ] JAX: jit, grad, vmap, функциональный стиль и explicit PRNG — дополнительная экосистема по потребности команды.

**Проверка:** переобучить сеть на маленьком batch, затем обучить на полном датасете; восстановить обучение из checkpoint и объяснить расхождения.

## 12. Архитектуры и представления — B, отдельные семейства C

1. [ ] CNN: convolution, padding, stride, receptive field, pooling, residual networks.
2. [ ] RNN, LSTM, GRU, backpropagation through time и ограничения длинных зависимостей.
3. [ ] Attention: query/key/value, scaled dot product, multi-head, masking и cross-attention.
4. [ ] Transformer: encoder, decoder, pre-norm/post-norm, FFN, residual stream.
5. [ ] Positional encodings, relative positions, RoPE и ограничения экстраполяции контекста.
6. [ ] MHA, MQA, GQA; влияние на качество и KV cache.
7. [ ] Mixture of Experts: routing, top-k experts, load balancing и коммуникации.
8. [ ] State-space models и hybrid architectures: идеи, вычислительные свойства и сравнение с attention.
9. [ ] Representation learning: contrastive objectives, metric learning, negative sampling и collapse.
10. [ ] Graph neural networks: message passing, equivariance, oversmoothing — C.

**Проверка:** реализовать небольшой decoder-only Transformer, указать форму каждого тензора и проверить отсутствие доступа к будущим токенам.

## 13. Языковые модели и pretraining — B

1. [ ] Tokenization: bytes, Unicode, BPE, unigram, vocabulary и special tokens.
2. [ ] Autoregressive и masked objectives, next-token prediction, perplexity и ограничения сравнения при разных токенизаторах.
3. [ ] Подготовка корпусов: filtering, deduplication, quality scoring, PII handling и benchmark decontamination.
4. [ ] Data mixtures, sampling weights, multilingual data, code data и curriculum.
5. [ ] Packing, document boundaries, attention masks, padding и нормировка loss по токенам.
6. [ ] Scaling laws, compute-optimal tradeoffs, ограничения экстраполяции и pilot runs.
7. [ ] Оценка числа параметров, training FLOPs, памяти и времени обучения.
8. [ ] Training stability: loss spikes, gradient norms, activation statistics, optimizer states и диагностика испорченных batch.
9. [ ] Long-context training, retrieval, position adaptation и оценка использования контекста.
10. [ ] Continual pretraining, catastrophic forgetting, distillation и model adaptation.
11. [ ] Memorization, contamination, hallucinations, in-context learning и различие знания/извлечения/следования инструкции.

**Проверка:** обучить небольшую LM с нуля, сравнить две смеси данных при одинаковом бюджете и объяснить ошибки на отложенных примерах.

## 14. Reinforcement learning, post-training и reasoning — B/C

1. [ ] MDP, states/actions/rewards, trajectories, discounted return, policies и value functions.
2. [ ] Bellman equations, dynamic programming, Monte Carlo, temporal difference, Q-learning.
3. [ ] Policy gradients, REINFORCE, actor-critic, advantage estimation и exploration.
4. [ ] On-policy/off-policy, importance sampling, offline RL и distribution mismatch.
5. [ ] PPO: clipped objective, value loss, KL control и типичные нестабильности.
6. [ ] Supervised fine-tuning: instruction data, chat format, loss masking, data quality.
7. [ ] Full fine-tuning, adapters, LoRA и quantized adaptation; стоимость и ограничения.
8. [ ] Preference data, reward modeling, pairwise objectives и annotator disagreement.
9. [ ] RLHF, RLAIF, Constitutional AI и DPO: цели, предположения и различия pipeline.
10. [ ] Обучение с проверяемыми наградами, group-relative policy optimization, outcome/process supervision.
11. [ ] Reward hacking, specification gaming, length bias, overoptimization и деградация разнообразия.
12. [ ] Reasoning: sampling, search, verifiers, best-of-N, test-time compute и сравнение при равном бюджете.
13. [ ] Agent RL: длинные траектории, credit assignment, среда, reset и воспроизводимость задач.

**Проверка:** провести SFT и небольшой preference-эксперимент; измерить целевое улучшение и регрессии на независимом наборе. Понимать RL глубже обязательно для специализации в post-training.

## 15. Оценка моделей и научный эксперимент — B

1. [ ] Формулировка проверяемой гипотезы и выбор измерения до запуска эксперимента.
2. [ ] Baselines, controlled comparisons, ablations, seed sensitivity и compute-matched comparisons.
3. [ ] Разделение development evals и закрытой итоговой проверки; предотвращение benchmark overfitting.
4. [ ] Benchmark contamination, near-duplicates, временные срезы и происхождение заданий.
5. [ ] Human evaluation: rubrics, blind comparisons, agreement и sampling strategy.
6. [ ] LLM-as-a-judge: position bias, verbosity bias, self-preference и калибровка по людям.
7. [ ] Calibration, abstention, uncertainty, factuality и selective prediction.
8. [ ] Оценка reasoning, кода, tool use и агентов; корректное применение pass@k и учёт бюджета попыток.
9. [ ] Slice analysis, robustness, adversarial cases, multilingual и long-context evaluations.
10. [ ] Практическая значимость, интервалы неопределённости, зависимые примеры и множественные сравнения.
11. [ ] Regression suites, воспроизводимый eval harness, versioning и анализ расхождений.

**Проверка:** построить оценивание, при котором улучшение нельзя получить простой утечкой, увеличением числа попыток или подстройкой под judge.

## 16. GPU и производительность — B обзор, C глубоко

1. [ ] GPU architecture: SM, warps, threads, blocks, tensor cores и occupancy.
2. [ ] Registers, shared memory, HBM, cache, coalescing и memory hierarchy.
3. [ ] Compute-bound и memory-bound операции; arithmetic intensity и roofline model.
4. [ ] GEMM, tiling, reductions, fused kernels и снижение обмена с памятью.
5. [ ] CUDA streams, events, asynchronous execution и корректное измерение времени.
6. [ ] PyTorch profiler, GPU profiling, kernel launch overhead и CPU bottlenecks.
7. [ ] CUDA или Triton: написать простой kernel и сравнить с эталоном.
8. [ ] FlashAttention: точное attention с экономией обращений к памяти; отличие от приближённого attention.
9. [ ] Activation checkpointing, recomputation, mixed precision и tradeoff память/вычисления.
10. [ ] Численная корректность оптимизаций; benchmark с warmup, синхронизацией и разными shapes.

**Проверка:** ускорить реальный bottleneck и показать throughput, peak memory, погрешность и область shapes, где ускорение работает.

## 17. Распределённое обучение — B обзор, C глубоко

1. [ ] Data parallelism, gradient synchronization, all-reduce, reduce-scatter и all-gather.
2. [ ] DDP, sharding optimizer states/gradients/parameters, ZeRO и FSDP.
3. [ ] Tensor, pipeline, sequence/context и expert parallelism.
4. [ ] Microbatches, pipeline bubbles, accumulation и глобальный batch size.
5. [ ] Interconnects, topology, communication/computation overlap и bandwidth bottlenecks.
6. [ ] NCCL, collective operations, stragglers, hangs и distributed debugging.
7. [ ] Cluster schedulers, resource allocation, quotas, preemption и job orchestration.
8. [ ] Distributed checkpointing, fault tolerance, recovery и корректность data order.
9. [ ] Scaling efficiency, tokens/sec, utilization, оценка MFU с явной методикой подсчёта.
10. [ ] Планирование больших запусков: пилот, бюджет, мониторинг, условия остановки и расследование сбоев.

**Проверка:** сравнить один и несколько GPU, объяснить потери масштабирования и проверить восстановление после остановки worker. Без доступа к кластеру изучи модель затрат и локальные multi-process опыты, явно обозначая масштаб проверки.

## 18. Inference, serving и ML в production — B/C

1. [ ] Autoregressive decoding: greedy, temperature, top-k/top-p, beam search и ограничения каждого метода.
2. [ ] Prefill/decode, KV cache, paged cache, continuous batching и prefix caching.
3. [ ] Quantization: weights/activations/KV, calibration, outliers, PTQ/QAT и компромисс качества.
4. [ ] Speculative decoding, draft models и условия сохранения целевого распределения.
5. [ ] Throughput, time to first token, inter-token latency, p50/p95/p99 и queueing.
6. [ ] Model parallel serving, replicas, autoscaling, admission control и memory budgeting.
7. [ ] APIs, streaming, cancellation, retries, rate limiting и versioned interfaces.
8. [ ] Containers, deployment, observability, model registry, canary/shadow rollout и rollback.
9. [ ] Data/model drift, feedback loops, online/offline metric mismatch и retraining.
10. [ ] Стоимость на запрос/токен/успешную задачу, capacity planning и SLO.
11. [ ] Безопасная загрузка артефактов, доступ к данным и весам, зависимости и изоляция исполнения.

**Проверка:** развернуть модель, нагрузить смешанными длинами запросов и показать качество, задержки, память и стоимость при росте нагрузки.

## 19. Retrieval, RAG и агенты — B/C

1. [ ] Sparse retrieval/BM25, dense embeddings, ANN indexes, hybrid search и reranking.
2. [ ] Chunking, metadata, indexing, freshness, permissions и document-level leakage.
3. [ ] Retrieval evaluation отдельно от generation; recall@k, attribution и groundedness.
4. [ ] Когда нужны retrieval, fine-tuning или изменение данных; проверка выбранного решения экспериментом.
5. [ ] Tool calling, structured outputs, schema validation и обработка ошибок инструментов.
6. [ ] Planning, state, memory, environment feedback, stop conditions и управление бюджетом.
7. [ ] Agent evaluation: success rate, trajectory analysis, повторные попытки и воспроизводимые environments.
8. [ ] Prompt injection, недоверенные документы, минимальные разрешения и sandboxing действий.
9. [ ] Долгие задачи: checkpoints, идемпотентные действия, recovery и человеческое подтверждение значимых операций.

**Проверка:** собрать RAG/агента с измеряемыми retrieval-ошибками, ошибками ответа и ошибками действий; сравнить с простым baseline.

## 20. Alignment, safety и интерпретируемость — B основы, C глубоко

1. [ ] Различие capabilities, alignment, robustness, security и governance.
2. [ ] Цели обучения и желаемое поведение; specification failures и Goodhart's law.
3. [ ] Helpful/honest/harmless поведение, sycophancy, calibrated uncertainty и качество отказов.
4. [ ] Threat models, red teaming, jailbreak evaluations и измерение ложных отказов.
5. [ ] Опасные возможности, misuse, deception и situational awareness как исследовательские вопросы с проверяемыми критериями.
6. [ ] Scalable oversight, debate, recursive supervision и weak-to-strong generalization — идеи и ограничения доказательств.
7. [ ] Mechanistic interpretability: circuits, features, superposition и sparse autoencoders.
8. [ ] Probes, activation patching, causal interventions и различие корреляционного объяснения и причинного механизма.
9. [ ] Faithfulness, completeness и пределы интерпретации; почему красивой визуализации недостаточно.
10. [ ] Privacy: memorization, membership inference, data extraction; differential privacy и её компромиссы.
11. [ ] Fairness, representational harms, выбор групп и конфликтующие определения справедливости.
12. [ ] Документирование ограничений, responsible disclosure и коммуникация неопределённости.

**Проверка:** воспроизвести небольшой эксперимент по интерпретируемости или safety; добавить контрольные условия и показать, какие выводы результат не поддерживает.

## 21. Мультимодальность и генеративные модели — C

1. [ ] Vision: classification, detection, segmentation, ViT, augmentation и image representations.
2. [ ] Image–text contrastive learning, multimodal encoders, projectors и cross-attention.
3. [ ] Audio: sampling, Fourier transform, spectrograms, speech recognition, synthesis и streaming.
4. [ ] Video: temporal modeling, frame sampling, temporal consistency и стоимость обработки.
5. [ ] Autoencoders, VAE, latent variables, ELBO и reparameterization.
6. [ ] GANs, adversarial training и mode collapse.
7. [ ] Diffusion: forward/reverse processes, denoising, score matching, noise schedules и sampling.
8. [ ] Flow matching, ODE/SDE viewpoint, guidance и latent generative models.
9. [ ] Multimodal data alignment, grounding, OCR, spatial reasoning и modality-specific evaluation.
10. [ ] Robotics/embodied AI: imitation learning, control, offline data, sim-to-real и безопасность физического действия — отдельная специализация.

**Проверка:** выбрать одно семейство, обучить небольшую модель и оценить её ошибки подходящими для модальности методами.

## 22. Исследовательское мышление и коммуникация — B

1. [ ] Чтение статьи: проблема, предположения, метод, baseline, протокол, результат и ограничения.
2. [ ] Вывод ключевых формул и восстановление деталей, отсутствующих в описании.
3. [ ] Поиск связанных работ, сопоставление противоречащих результатов и ведение библиографии.
4. [ ] Выбор вопроса по значимости, проверяемости, стоимости и доступности данных.
5. [ ] Разделение exploration и confirmation; заранее определённые критерии успеха/остановки.
6. [ ] Воспроизведение результата, поиск источников расхождения и честная фиксация отрицательных результатов.
7. [ ] Исследовательский журнал, таблицы запусков, experiment tracking и сохранение артефактов.
8. [ ] Написание технического отчёта: контекст, гипотеза, метод, результаты, ограничения и следующий эксперимент.
9. [ ] Рабочий английский: статьи, документация, обсуждение дизайна, письменные отчёты и презентации.
10. [ ] Совместная работа: code review, понятные PR, обсуждение разногласий и передача результатов коллегам.
11. [ ] Управление временем и вычислительным бюджетом; выбор простого эксперимента, различающего гипотезы.

**Проверка:** другой человек запускает твой эксперимент по README и понимает, что именно подтверждено данными.

## 23. В каком порядке учиться

1. **1. Базовый язык**
   - Разделы: 3–7.
   - Условие перехода: Уверенные вычисления, Python, градиенты и простые вероятностные задачи.

2. **2. Полный цикл ML**
   - Разделы: 8–10.
   - Условие перехода: Самостоятельная задача с честным split, baseline и анализом ошибок.

3. **3. Нейросети**
   - Разделы: 11–12.
   - Условие перехода: Самостоятельный training loop, диагностика обучения, небольшой Transformer.

4. **4. Языковые модели**
   - Разделы: 13, 15, основы 14.
   - Условие перехода: Маленькое pretraining, SFT и воспроизводимое оценивание.

5. **5. Инженерная зрелость**
   - Разделы: Основы 16–19.
   - Условие перехода: Профиль производительности, serving и понимание распределённого обучения.

6. **6. Глубокая специализация**
   - Разделы: Выбранные 14–21.
   - Условие перехода: Сильный законченный проект с измеримым результатом.

7. **7. Самостоятельная работа**
   - Разделы: 22, 24–26.
   - Условие перехода: Исследование или система, которую могут проверить другие.


Статистику, английский, качество кода и экспериментальную дисциплину развивай на всех этапах. Не откладывай проекты до прочтения всей карты. Срок зависит от стартового уровня и доступного времени; переходи по результатам, а не по календарю.

Для **Research Engineer** углубляй 11–17 и 22. Для **Research Scientist** — 4–5, 10, 15, 22 и выбранную исследовательскую область. Для **ML Systems** — 6–7, 16–18. Для **Alignment/Safety** — 14–15, 20, 22. Для **Applied ML** — 8–9, 15, 18–19. Это рекомендуемые маршруты, а не формальные требования работодателей.

## 24. Проекты, которыми подтверждается уровень

Для каждого проекта нужны исходники, инструкция запуска, версия данных, конфиг, метрики, бюджет вычислений, анализ ошибок и ограничения. Указывай фактический масштаб эксперимента.

1. **Классический ML end-to-end.** Реальные данные, корректное разбиение, простая модель и бустинг, сравнение с неопределённостью, небольшой сервис и мониторинг входных данных.
2. **Autograd и нейросеть с нуля.** Маленький движок автоматического дифференцирования, MLP, gradient checks и сравнение с PyTorch. Реализацию можно сделать на Rust.
3. **Маленькая языковая модель.** Токенизатор, Transformer, обучение, checkpoint/resume, generation и eval. Сравнить минимум две осмысленные гипотезы при одинаковом бюджете.
4. **Post-training.** SFT и один метод preference optimization на небольшой модели; отдельные train/eval, контроль качества и анализ регрессий.
5. **Проект по производительности.** Найти bottleneck, ускорить kernel или serving; измерить корректность, latency, throughput и память. Для systems-направления добавить multi-GPU опыт.
6. **Воспроизведение статьи.** Восстановить основной эксперимент доступного масштаба, провести ablations, описать несовпадения и проверить собственную гипотезу.
7. **Проект специализации.** Interpretability, safety evals, multimodal learning, RL, data quality или infrastructure. Сформулировать новый вопрос и получить результат, полезный другим.

Первые три проекта образуют хороший учебный фундамент. Дальше выбери два-три под целевую роль и доведи до качества внешнего ревью. Небольшой тщательно проверенный эксперимент ценен, но не выдавай его за опыт эксплуатации большого кластера. Для личного обучения не требуется оплачивать pretraining frontier-модели: используй малые модели, короткие запуски и явные ограничения масштаба.

## 25. Материалы для изучения

Ниже — названия фундаментальных источников для поиска в авторских и официальных изданиях. Это опорная библиотека; проходить каждый источник целиком не требуется.

1. **Линейная алгебра**
   - Источники: Gilbert Strang, Introduction to Linear Algebra; Mathematics for Machine Learning, Deisenroth/Faisal/Ong.
   - Как использовать: Решать задачи, связывать формулы с тензорным кодом.

2. **Вероятность и статистика**
   - Источники: Blitzstein/Hwang, Introduction to Probability; Wasserman, All of Statistics.
   - Как использовать: Выводить оценки и проверять их симуляциями.

3. **Оптимизация**
   - Источники: Boyd/Vandenberghe, Convex Optimization.
   - Как использовать: Начать с геометрии, градиентов и ограниченной оптимизации.

4. **Классическое ML**
   - Источники: James et al., An Introduction to Statistical Learning; Murphy, Probabilistic Machine Learning.
   - Как использовать: Реализовывать baseline и проверять предположения методов.

5. **Deep learning**
   - Источники: Goodfellow/Bengio/Courville, Deep Learning; Dive into Deep Learning.
   - Как использовать: Совмещать главы с маленькими обучаемыми моделями.

6. **Практика нейросетей**
   - Источники: Andrej Karpathy, Neural Networks: Zero to Hero; официальные tutorials PyTorch.
   - Как использовать: После повторения переписать ключевые части самостоятельно.

7. **RL**
   - Источники: Sutton/Barto, Reinforcement Learning: An Introduction.
   - Как использовать: Сначала tabular methods, затем policy gradients.

8. **Системы**
   - Источники: Hennessy/Patterson, Computer Architecture: A Quantitative Approach; документация CUDA, NCCL, PyTorch distributed.
   - Как использовать: Подтверждать понимание профилированием и измерениями.


**Опорные статьи для самостоятельного разбора:** Attention Is All You Need; BERT; Language Models are Few-Shot Learners; Scaling Laws for Neural Language Models; Training Compute-Optimal Large Language Models; FlashAttention; ZeRO; LoRA; Training Language Models to Follow Instructions with Human Feedback; Direct Preference Optimization; Constitutional AI; CLIP; Denoising Diffusion Probabilistic Models; Toy Models of Superposition.

Читайте статьи после соответствующих основ. Для каждой запиши: что изменили, с чем сравнили, какой бюджет использовали, что воспроизводится, какие ограничения остались. Конкретные API, поддерживаемое оборудование и инструкции запуска проверяй в актуальной официальной документации используемой версии.

## 26. Готовность к сильной команде и собеседованиям

1. [ ] Могу без подсказки написать training loop, найти ошибку в shapes/masking и объяснить backpropagation.
2. [ ] Могу решить задачу по вероятности, статистике или линейной алгебре и обозначить предположения решения.
3. [ ] Могу выбрать baseline, метрику и разбиение данных под новую задачу.
4. [ ] Могу диагностировать NaN, overfitting, data leakage, медленный loader и нехватку GPU-памяти.
5. [ ] Могу оценить порядок training/inference compute и памяти параметров, градиентов, optimizer states, activations и KV cache.
6. [ ] Могу прочитать статью, воспроизвести существенную часть и объяснить, что осталось неподтверждённым.
7. [ ] Могу предложить следующий эксперимент, который различает несколько причин наблюдаемого эффекта.
8. [ ] Могу спроектировать систему с ограничениями качества, latency, стоимости, надёжности и приватности.
9. [ ] Имею глубокую специализацию и могу обсуждать её компромиссы на уровне реализации и эксперимента.
10. [ ] Есть законченные проекты с воспроизводимыми результатами и понятным личным вкладом.
11. [ ] Могу защитить решения, признать ошибку и объяснить работу устно и письменно на английском.
12. [ ] Изучил актуальную вакансию и могу сопоставить каждое существенное требование с конкретным примером своей работы.

Для подготовки тренируй четыре формата: программирование, ML/математика, проектирование систем и подробный разбор собственного проекта. Для исследовательской роли добавь обсуждение статьи и план нового исследования. Это полезные форматы тренировки, а не обещание конкретной структуры интервью.

Сильный профессиональный результат — самостоятельное движение от неясной задачи к проверенному выводу или надёжной системе, понимание ограничений и способность улучшать результат вместе с командой.
