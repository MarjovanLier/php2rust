<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Attributes\Initialize;
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

final class Initialized extends Model
{
    protected $table = 'items';

    #[Initialize]
    protected function prepare(): void
    {
    }
}

final class Lifecycle extends Model
{
    protected $table = 'items';

    protected function initializeNamed(): void
    {
    }
}

final class Nullable extends Model
{
    protected $table = 'items';

    protected function name(): ?Attribute
    {
        return null;
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

// PHP rejects these, and Mago does not report them.
final final class Twice extends Model
{
    protected $table = 'items';
}

final class Tables extends Model
{
    protected $table = 'nothing';

    protected $table = 'items';
}

final class Stamps extends Model
{
    protected $table = 'items';

    public $timestamps = true;

    public $timestamps = false;
}

final class Casted extends Model
{
    protected $table = 'items';

    protected function casts(): array
    {
        return [];
    }

    protected function CASTS(): array
    {
        return [];
    }
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
