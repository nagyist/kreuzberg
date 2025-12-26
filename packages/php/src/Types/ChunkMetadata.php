<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Chunk metadata describing offsets within the original document.
 *
 * @property-read int $byteStart Starting byte offset
 * @property-read int $byteEnd Ending byte offset
 * @property-read int|null $tokenCount Number of tokens in chunk
 * @property-read int $chunkIndex Chunk index (0-based)
 * @property-read int $totalChunks Total number of chunks
 * @property-read int|null $firstPage First page number in chunk
 * @property-read int|null $lastPage Last page number in chunk
 */
readonly class ChunkMetadata
{
    public function __construct(
        public int $byteStart,
        public int $byteEnd,
        public ?int $tokenCount,
        public int $chunkIndex,
        public int $totalChunks,
        public ?int $firstPage = null,
        public ?int $lastPage = null,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        return new self(
            byteStart: $data['byte_start'] ?? 0,
            byteEnd: $data['byte_end'] ?? 0,
            tokenCount: $data['token_count'] ?? null,
            chunkIndex: $data['chunk_index'] ?? 0,
            totalChunks: $data['total_chunks'] ?? 0,
            firstPage: $data['first_page'] ?? null,
            lastPage: $data['last_page'] ?? null,
        );
    }
}
