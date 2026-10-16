<?php

declare(strict_types=1);

namespace App;

use App\Rules\NoSqlInjectionRule;
use Illuminate\Support\Facades\Gate;
use Illuminate\Validation\Rule;
use Spatie\LaravelData\Data;
use Spatie\LaravelData\Support\Validation\ValidationContext;

final class Invoice extends Data
{
    public function __construct(
        public string $number,
        public int $lines,
    ) {}

    // No file declares Rule or NoSqlInjectionRule, as without an --include of Laravel.
    /** @return array<string, list<mixed>> */
    public static function rules(?ValidationContext $context = null): array
    {
        return [
            'number' => ['required', Rule::unique('invoices')->ignore($context?->payload['id'] ?? null)],
            'lines' => ['integer', 'min:1', new NoSqlInjectionRule()],
        ];
    }

    // Mago checks it against the tag messages(...$args), which PHP never reads.
    /** @return array<string, string> */
    static function MESSAGES(): array
    {
        return ['number.required' => 'A number is needed.'];
    }

    /** @return array<string, string> */
    public static function attributes(): array
    {
        return ['number' => 'invoice number'];
    }

    final public static function stopOnFirstFailure(): bool
    {
        return true;
    }

    // An Eloquent model answers query() through __callStatic, which Mago does not see.
    public static function redirect(): string
    {
        self::query();
        return 'invoices.index';
    }

    public static function redirectRoute(): string
    {
        return 'invoices.index';
    }

    public static function errorBag(): string
    {
        return 'invoice';
    }

    public static function authorize(): bool
    {
        return Gate::allows('create-invoice') === true;
    }

    public function total(int $price): int
    {
        return $this->lines * $price;
    }
}

$invoice = new Invoice('A-1', 3);
echo $invoice->number, ' ', $invoice->total(4), "\n";
