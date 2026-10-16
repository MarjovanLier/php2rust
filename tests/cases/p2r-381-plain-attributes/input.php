<?php

declare(strict_types=1);

namespace App;

use Spatie\LaravelData\Attributes\MapInputName;
use Spatie\LaravelData\Attributes\MapOutputName;
use Spatie\LaravelData\Attributes\Validation\Max;

interface Named
{
    public function name(): string;
}

// laravel-data reads its attributes only on a class that extends Data.
#[MapOutputName('snake')]
final class Value implements Named
{
    public function __construct(
        #[MapInputName('data.code'), Max(10)]
        public string $code,
        #[MapInputName('data.label')]
        public string $label,
    ) {}

    public function name(): string
    {
        return $this->code . ':' . $this->label;
    }
}

#[MapInputName('data')]
readonly class Total
{
    public function __construct(
        #[Max(100)]
        public int $sum,
    ) {}
}

$value = new Value('A', 'first');
$total = new Total(7);
echo $value->name(), ' ', $total->sum, "\n";
