<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Text chunk with optional embedding vector.
 *
 * @property-read string $content Chunk text content
 * @property-read array<float>|null $embedding Embedding vector
 * @property-read ChunkMetadata $metadata Chunk metadata
 */
readonly class Chunk
{
    /**
     * @param array<float>|null $embedding
     */
    public function __construct(
        public string $content,
        public ?array $embedding,
        public ChunkMetadata $metadata,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        return new self(
            content: $data['content'] ?? '',
            embedding: $data['embedding'] ?? null,
            metadata: ChunkMetadata::fromArray($data['metadata'] ?? []),
        );
    }
}
