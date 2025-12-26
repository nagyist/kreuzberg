<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Language detection configuration.
 */
readonly class LanguageDetectionConfig
{
    public function __construct(
        public bool $enabled = false,
        public ?int $maxLanguages = null,
        public ?float $confidenceThreshold = null,
    ) {
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return array_filter([
            'enabled' => $this->enabled,
            'max_languages' => $this->maxLanguages,
            'confidence_threshold' => $this->confidenceThreshold,
        ], static fn ($value): bool => $value !== null);
    }
}
