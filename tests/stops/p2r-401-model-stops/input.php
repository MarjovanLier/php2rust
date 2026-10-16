<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Attributes\ObservedBy;
use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

trait Named
{
}

interface Labelled
{
}

// Each of these takes the struct away, because PHP may read an attribute through it.
#[ObservedBy([Observed::class])]
final class Observed extends Model
{
    protected $table = 'items';
}

final class Accessed extends Model
{
    protected $table = 'items';

    public function getNameAttribute(string $value): string
    {
        return strtoupper($value);
    }
}

final class Attributed extends Model
{
    protected $table = 'items';

    protected function name(): Attribute
    {
        return new Attribute();
    }
}

final class Booted extends Model
{
    protected $table = 'items';

    protected static function booted(): void
    {
    }
}

final class Constant extends Model
{
    protected $table = 'items';

    public const KIND = 'x';
}

final class Configured extends Model
{
    protected $table = 'items';

    protected $connection = 'other';
}

final class Traited extends Model
{
    use Named;

    protected $table = 'items';
}

abstract class Abstracted extends Model
{
    protected $table = 'items';
}

final class Interfaced extends Model implements Labelled
{
    protected $table = 'items';
}

final class Missing extends Model
{
}

final class Miscast extends Model
{
    protected $table = 'items';

    protected $casts = ['nothing' => 'string'];
}

final class Pair extends Model
{
}

// A relation stops, and it leaves the struct, so the read below converts.
final class Related extends Model
{
    protected $table = 'items';

    public function parent(): BelongsTo
    {
        return new BelongsTo();
    }
}

function relatedName(Related $related): string
{
    return $related->name;
}

// A model without a struct has no property to read.
function observedName(Observed $observed): string
{
    return $observed->name;
}
