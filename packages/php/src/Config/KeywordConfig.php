<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Keyword extraction configuration.
 */
readonly class KeywordConfig
{
    public function __construct(
        public bool $enabled = false,
        public string $algorithm = 'rake',
        public ?int $maxKeywords = null,
    ) {
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return array_filter([
            'enabled' => $this->enabled,
            'algorithm' => $this->algorithm,
            'max_keywords' => $this->maxKeywords,
        ], static fn ($value): bool => $value !== null);
    }
}
