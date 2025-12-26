<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Document metadata.
 *
 * All fields are optional and depend on the file format, extraction configuration,
 * and postprocessors enabled.
 *
 * @property-read string|null $language Document language (ISO 639-1 code)
 * @property-read string|null $date Document date (ISO 8601 format)
 * @property-read string|null $subject Document subject
 * @property-read string|null $formatType Format discriminator ("pdf", "excel", "email", etc.)
 * @property-read string|null $title Document title
 * @property-read array<string>|null $authors Document authors
 * @property-read array<string>|null $keywords Document keywords
 * @property-read string|null $createdAt Creation date (ISO 8601)
 * @property-read string|null $modifiedAt Modification date (ISO 8601)
 * @property-read string|null $createdBy Creator/application name
 * @property-read string|null $producer Producer/generator
 * @property-read int|null $pageCount Number of pages
 * @property-read array<string, mixed> $custom Additional custom metadata from postprocessors
 */
readonly class Metadata
{
    /**
     * @param array<string>|null $authors
     * @param array<string>|null $keywords
     * @param array<string, mixed> $custom
     */
    public function __construct(
        public ?string $language = null,
        public ?string $date = null,
        public ?string $subject = null,
        public ?string $formatType = null,
        public ?string $title = null,
        public ?array $authors = null,
        public ?array $keywords = null,
        public ?string $createdAt = null,
        public ?string $modifiedAt = null,
        public ?string $createdBy = null,
        public ?string $producer = null,
        public ?int $pageCount = null,
        public array $custom = [],
    ) {
    }

    /**
     * Create Metadata from array returned by extension.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        // Known fields
        $knownFields = [
            'language',
            'date',
            'subject',
            'format_type',
            'title',
            'authors',
            'keywords',
            'created_at',
            'modified_at',
            'created_by',
            'producer',
            'page_count',
        ];

        // Extract known fields
        $metadata = [
            'language' => $data['language'] ?? null,
            'date' => $data['date'] ?? null,
            'subject' => $data['subject'] ?? null,
            'formatType' => $data['format_type'] ?? null,
            'title' => $data['title'] ?? null,
            'authors' => $data['authors'] ?? null,
            'keywords' => $data['keywords'] ?? null,
            'createdAt' => $data['created_at'] ?? null,
            'modifiedAt' => $data['modified_at'] ?? null,
            'createdBy' => $data['created_by'] ?? null,
            'producer' => $data['producer'] ?? null,
            'pageCount' => $data['page_count'] ?? null,
        ];

        // Collect custom fields (anything not in known fields)
        $custom = [];
        foreach ($data as $key => $value) {
            if (!in_array($key, $knownFields, true)) {
                $custom[$key] = $value;
            }
        }

        return new self(...$metadata, custom: $custom);
    }

    /**
     * Get a custom metadata field.
     *
     * @param string $key
     * @return mixed
     */
    public function getCustom(string $key): mixed
    {
        return $this->custom[$key] ?? null;
    }

    /**
     * Check if a custom metadata field exists.
     */
    public function hasCustom(string $key): bool
    {
        return isset($this->custom[$key]);
    }
}
