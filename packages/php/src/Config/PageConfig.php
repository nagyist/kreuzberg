<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Page extraction configuration.
 */
readonly class PageConfig
{
    public function __construct(
        public bool $extractPages = false,
        public bool $insertPageMarkers = false,
        public string $markerFormat = '--- Page {page_number} ---',
    ) {
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return [
            'extract_pages' => $this->extractPages,
            'insert_page_markers' => $this->insertPageMarkers,
            'marker_format' => $this->markerFormat,
        ];
    }
}
