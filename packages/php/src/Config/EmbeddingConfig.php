<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Embedding generation configuration.
 */
readonly class EmbeddingConfig
{
    public function __construct(
        public string $model = 'all-MiniLM-L6-v2',
        public bool $normalize = true,
        public ?int $batchSize = null,
    ) {
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return array_filter([
            'model' => $this->model,
            'normalize' => $this->normalize,
            'batch_size' => $this->batchSize,
        ], static fn ($value): bool => $value !== null);
    }
}
