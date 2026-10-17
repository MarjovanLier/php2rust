<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Concerns\FillsTyped;
use Illuminate\Database\Eloquent\Model;

// Mago's findings that P2R-490 does not drop still stop (P2R-491): a cast to a class that
// does not exist inside a configuration property, the same docblock finding on a model
// property that P2R-401 does not read, and on a class that is no model.
final class Code extends Model
{
    // The parent's native type left out, which PHP refuses; Mago puts this variant of the
    // finding inside the declaration.
    protected $hidden = ['code'];

    /**
     * @var array<string, string>
     */
    protected $casts = ['code' => Missing::class];

    /**
     * @var list<string>
     */
    protected $appends = ['label'];
}

// A type that conflicts with a trait the model uses, which PHP refuses; Mago puts this
// finding on the declaration.
final class Conflict extends Model
{
    use FillsTyped;

    protected $table = 'codes';

    protected $fillable = ['code'];
}

// Another finding in the docblock of an admitted property still stops.
final class Docblocked extends Model
{
    protected $table = 'codes';

    /**
     * @var list<Missing>
     */
    protected $fillable = ['code'];
}

class Base
{
    /**
     * @var array<int, string>
     */
    protected $items = [];
}

final class Child extends Base
{
    /**
     * @var list<string>
     */
    protected $items = ['a'];
}
