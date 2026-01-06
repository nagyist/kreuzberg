<?php

declare(strict_types=1);

namespace Kreuzberg\Tests\Unit\Config;

use Kreuzberg\Config\ImagePreprocessingConfig;
use PHPUnit\Framework\Attributes\CoversClass;
use PHPUnit\Framework\Attributes\Group;
use PHPUnit\Framework\Attributes\Test;
use PHPUnit\Framework\TestCase;

/**
 * Unit tests for ImagePreprocessingConfig readonly class.
 *
 * Tests construction, serialization, factory methods, readonly enforcement,
 * and handling of mixed property types (int, bool).
 *
 * Test Coverage:
 * - Construction with default values
 * - Construction with custom values
 * - toArray() serialization
 * - fromArray() factory method
 * - fromJson() factory method
 * - toJson() serialization
 * - Readonly enforcement
 * - Type coercion
 * - Invalid JSON handling
 * - Round-trip serialization
 */
#[CoversClass(ImagePreprocessingConfig::class)]
#[Group('unit')]
#[Group('config')]
final class ImagePreprocessingConfigTest extends TestCase
{
    #[Test]
    public function it_creates_with_default_values(): void
    {
        $config = new ImagePreprocessingConfig();

        $this->assertSame(300, $config->targetDpi);
        $this->assertSame(90, $config->quality);
        $this->assertFalse($config->grayscale);
        $this->assertFalse($config->denoise);
        $this->assertFalse($config->deskew);
        $this->assertFalse($config->removeBackground);
    }

    #[Test]
    public function it_creates_with_custom_values(): void
    {
        $config = new ImagePreprocessingConfig(
            targetDpi: 600,
            quality: 95,
            grayscale: true,
            denoise: true,
            deskew: true,
            removeBackground: true,
        );

        $this->assertSame(600, $config->targetDpi);
        $this->assertSame(95, $config->quality);
        $this->assertTrue($config->grayscale);
        $this->assertTrue($config->denoise);
        $this->assertTrue($config->deskew);
        $this->assertTrue($config->removeBackground);
    }

    #[Test]
    public function it_serializes_to_array(): void
    {
        $config = new ImagePreprocessingConfig(
            targetDpi: 200,
            quality: 80,
            denoise: true,
        );
        $array = $config->toArray();

        $this->assertIsArray($array);
        $this->assertArrayHasKey('target_dpi', $array);
        $this->assertArrayHasKey('quality', $array);
        $this->assertArrayHasKey('denoise', $array);
        $this->assertSame(200, $array['target_dpi']);
        $this->assertSame(80, $array['quality']);
        $this->assertTrue($array['denoise']);
    }

    #[Test]
    public function it_creates_from_array_with_defaults(): void
    {
        $config = ImagePreprocessingConfig::fromArray([]);

        $this->assertSame(300, $config->targetDpi);
        $this->assertSame(90, $config->quality);
        $this->assertFalse($config->grayscale);
        $this->assertFalse($config->deskew);
    }

    #[Test]
    public function it_creates_from_array_with_all_fields(): void
    {
        $data = [
            'target_dpi' => 600,
            'quality' => 95,
            'grayscale' => true,
            'denoise' => true,
            'deskew' => true,
            'remove_background' => true,
        ];
        $config = ImagePreprocessingConfig::fromArray($data);

        $this->assertSame(600, $config->targetDpi);
        $this->assertSame(95, $config->quality);
        $this->assertTrue($config->grayscale);
        $this->assertTrue($config->denoise);
        $this->assertTrue($config->deskew);
        $this->assertTrue($config->removeBackground);
    }

    #[Test]
    public function it_serializes_to_json(): void
    {
        $config = new ImagePreprocessingConfig(
            targetDpi: 300,
            quality: 85,
            grayscale: true,
            denoise: true,
        );
        $json = $config->toJson();

        $this->assertJson($json);
        $decoded = json_decode($json, true);

        $this->assertSame(300, $decoded['target_dpi']);
        $this->assertSame(85, $decoded['quality']);
        $this->assertTrue($decoded['grayscale']);
        $this->assertTrue($decoded['denoise']);
    }

    #[Test]
    public function it_creates_from_json(): void
    {
        $json = json_encode([
            'target_dpi' => 150,
            'quality' => 75,
            'deskew' => true,
            'denoise' => true,
        ]);
        $config = ImagePreprocessingConfig::fromJson($json);

        $this->assertSame(150, $config->targetDpi);
        $this->assertSame(75, $config->quality);
        $this->assertTrue($config->deskew);
        $this->assertTrue($config->denoise);
    }

    #[Test]
    public function it_round_trips_through_json(): void
    {
        $original = new ImagePreprocessingConfig(
            targetDpi: 300,
            quality: 90,
            grayscale: true,
            denoise: true,
            deskew: true,
            removeBackground: true,
        );

        $json = $original->toJson();
        $restored = ImagePreprocessingConfig::fromJson($json);

        $this->assertSame($original->targetDpi, $restored->targetDpi);
        $this->assertSame($original->quality, $restored->quality);
        $this->assertSame($original->grayscale, $restored->grayscale);
        $this->assertSame($original->denoise, $restored->denoise);
        $this->assertSame($original->deskew, $restored->deskew);
        $this->assertSame($original->removeBackground, $restored->removeBackground);
    }

    #[Test]
    public function it_throws_on_invalid_json(): void
    {
        $this->expectException(\InvalidArgumentException::class);
        $this->expectExceptionMessage('Invalid JSON');

        ImagePreprocessingConfig::fromJson('{ invalid }');
    }

    #[Test]
    public function it_enforces_readonly_on_target_dpi_property(): void
    {
        $this->expectException(\Error::class);

        $config = new ImagePreprocessingConfig(targetDpi: 300);
        $config->targetDpi = 200;
    }

    #[Test]
    public function it_enforces_readonly_on_quality_property(): void
    {
        $this->expectException(\Error::class);

        $config = new ImagePreprocessingConfig(quality: 90);
        $config->quality = 80;
    }

    #[Test]
    public function it_enforces_readonly_on_grayscale_property(): void
    {
        $this->expectException(\Error::class);

        $config = new ImagePreprocessingConfig(grayscale: true);
        $config->grayscale = false;
    }

    #[Test]
    public function it_creates_from_file(): void
    {
        $tempFile = tempnam(sys_get_temp_dir(), 'imgprep_');
        if ($tempFile === false) {
            $this->markTestSkipped('Unable to create temporary file');
        }

        try {
            file_put_contents($tempFile, json_encode([
                'target_dpi' => 300,
                'quality' => 85,
            ]));

            $config = ImagePreprocessingConfig::fromFile($tempFile);

            $this->assertSame(300, $config->targetDpi);
            $this->assertSame(85, $config->quality);
        } finally {
            if (file_exists($tempFile)) {
                unlink($tempFile);
            }
        }
    }

    #[Test]
    public function it_throws_when_file_not_found(): void
    {
        $this->expectException(\InvalidArgumentException::class);
        $this->expectExceptionMessage('File not found');

        ImagePreprocessingConfig::fromFile('/nonexistent/path/config.json');
    }

    #[Test]
    public function it_handles_type_coercion_for_target_dpi(): void
    {
        $data = ['target_dpi' => '300'];
        $config = ImagePreprocessingConfig::fromArray($data);

        $this->assertIsInt($config->targetDpi);
        $this->assertSame(300, $config->targetDpi);
    }

    #[Test]
    public function it_handles_type_coercion_for_quality(): void
    {
        $data = ['quality' => '80'];
        $config = ImagePreprocessingConfig::fromArray($data);

        $this->assertIsInt($config->quality);
        $this->assertSame(80, $config->quality);
    }

    #[Test]
    public function it_handles_type_coercion_for_bool_values(): void
    {
        $data = [
            'grayscale' => 1,
            'denoise' => '1',
            'deskew' => 'true',
        ];
        $config = ImagePreprocessingConfig::fromArray($data);

        $this->assertIsBool($config->grayscale);
        $this->assertTrue($config->grayscale);
        $this->assertIsBool($config->denoise);
        $this->assertTrue($config->denoise);
        $this->assertIsBool($config->deskew);
        $this->assertTrue($config->deskew);
    }
}
