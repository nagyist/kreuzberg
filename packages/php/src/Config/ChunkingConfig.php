<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Text chunking configuration.
 */
readonly class ChunkingConfig
{
    public function __construct(
        public int $maxChunkSize = 512,
        public int $chunkOverlap = 50,
        public bool $respectSentences = true,
        public bool $respectParagraphs = true,
    ) {
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return [
            'max_chunk_size' => $this->maxChunkSize,
            'chunk_overlap' => $this->chunkOverlap,
            'respect_sentences' => $this->respectSentences,
            'respect_paragraphs' => $this->respectParagraphs,
        ];
    }
}
