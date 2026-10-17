<?php

declare(strict_types=1);

namespace App;

use Illuminate\Support\Facades\Gate;
use Illuminate\Support\Facades\Log;
use Illuminate\Validation\Rule;
use Spatie\LaravelData\Data;
use Spatie\LaravelData\Support\Validation\ValidationContext;

// Each hook keeps to the closed list of P2R-510, so the hooks are left out and the class
// converts. No file declares Gate, Log, Rule or the rule classes, as without an --include
// of Laravel.
final class Invoice extends Data
{
    public const string KIND = 'invoice';

    public function __construct(
        public string $number,
        public int $lines,
    ) {}

    /** @return array<string, list<mixed>> */
    public static function rules(?ValidationContext $context = null): array
    {
        return [
            'number' => ['required', 'string', 'max:20', new UpperCase, new MaxLength(20)],
            'lines' => [
                'integer',
                Rule::in([1, 2, 3]),
                Rule::unique('invoices')->ignore($context?->payload['id'] ?? null),
            ],
            'code' => [
                static function (string $attribute, mixed $value, \Closure $fail): void {
                    if (! is_string($value) && ! is_int($value) || $value === '') {
                        $fail("The {$attribute} field must be a string or an integer, not $attribute.");
                    }
                },
            ],
            'items' => [
                function (string $attribute, mixed $value, callable $fail): void {
                    if (! is_array($value)) {
                        return;
                    }

                    foreach ($value as $key => $item) {
                        if (! is_string($key)) {
                            continue;
                        }

                        if ($attribute == '' || strlen($key) < 1 || strlen($key) > 9 || $key != 'x') {
                            $fail('The item ' . $key . ' is not valid.');
                        }

                        if (strlen($key) <= 0 || strlen($key) >= 10 || $key !== $attribute) {
                            $fail(sprintf('The item %s is too long.', $key));
                        }
                    }
                },
            ],
            'kind' => [Invoice::class, Invoice::KIND],
        ];
    }

    /** @return array<string, string> */
    public static function messages(): array
    {
        return ['number.required' => 'A number is needed.', 'lines' => sprintf('%d lines', 3)];
    }

    public static function authorize(): bool
    {
        // An Eloquent model answers query() through __callStatic, which Mago does not see.
        self::query();
        static::query();
        if (Gate::allows('log-invoice') === true) {
            Log::info('authorising an invoice');
        }

        foreach (['view', 'create'] as $ability) {
            Log::info($ability);
        }

        return Gate::allows('create-invoice') === true;
    }

    public static function stopOnFirstFailure(): bool
    {
        return true;
    }

    public static function redirect(): string
    {
        return 'invoices.' . 'index';
    }

    public static function redirectRoute(): string
    {
        return "invoices.index";
    }

    public static function errorBag(): string
    {
        return Invoice::KIND;
    }

    public function total(int $price): int
    {
        return $this->lines * $price;
    }
}

$invoice = new Invoice('A-1', 3);
echo $invoice->number, ' ', $invoice->total(4), ' ', Invoice::KIND, "\n";
