import {
  Component,
  ChangeDetectionStrategy,
  input,
  forwardRef,
  Provider,
  ChangeDetectorRef,
  inject,
} from '@angular/core';
import { ControlValueAccessor, NG_VALUE_ACCESSOR } from '@angular/forms';

let nextId = 0;

/** Provider for ControlValueAccessor */
export const APP_INPUT_VALUE_ACCESSOR: Provider = {
  provide: NG_VALUE_ACCESSOR,
  useExisting: forwardRef(() => AppInputComponent),
  multi: true,
};

/**
 * A standard input/textarea/select component following GitHub Primer styling.
 */
@Component({
  selector: 'app-input',
  providers: [APP_INPUT_VALUE_ACCESSOR],
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    @if (label()) {
      <label class="input-label" [for]="inputId()">{{ label() }}</label>
    }

    @if (type() === 'textarea') {
      <textarea
        class="form-control"
        [id]="inputId()"
        [placeholder]="placeholder()"
        [disabled]="isDisabled"
        [value]="value"
        (input)="onInputChange($event)"
        (blur)="onTouched()"
        rows="3"
      ></textarea>
    } @else if (type() === 'select') {
      <select
        class="form-select"
        [id]="inputId()"
        [disabled]="isDisabled"
        [value]="value"
        (change)="onSelectChange($event)"
        (blur)="onTouched()"
      >
        @for (option of options(); track option.value) {
          <option [value]="option.value">{{ option.label }}</option>
        }
      </select>
    } @else {
      <input
        class="form-control"
        [type]="type()"
        [id]="inputId()"
        [placeholder]="placeholder()"
        [disabled]="isDisabled"
        [value]="value"
        (input)="onInputChange($event)"
        (blur)="onTouched()"
      />
    }
  `,
  styles: `
    :host {
      display: flex;
      flex-direction: column;
      gap: 4px;
    }

    .input-label {
      font-size: 14px;
      font-weight: 600;
      color: var(--color-fg-default);
    }

    .form-control,
    .form-select {
      padding: 5px 12px;
      font-size: 14px;
      line-height: 20px;
      color: var(--color-fg-default);
      background-color: var(--color-canvas-default);
      border: 1px solid var(--color-border-default);
      border-radius: var(--border-radius-1);
      outline: none;
      transition:
        border-color 0.1s ease-in-out,
        box-shadow 0.1s ease-in-out;
      width: 100%;
      font-family: inherit;
      box-sizing: border-box;
    }

    .form-control:focus,
    .form-select:focus {
      border-color: var(--color-accent-fg);
      box-shadow: 0 0 0 3px rgba(9, 105, 218, 0.3);
    }

    .form-control:disabled,
    .form-select:disabled {
      color: var(--color-fg-muted);
      background-color: var(--color-canvas-subtle);
      cursor: not-allowed;
    }
  `,
})
export class AppInputComponent implements ControlValueAccessor {
  /** Global counter for input ids */
  static nextId = 0;

  /** Input type (text, password, textarea, select, etc) */
  type = input<string>('text');

  /** Label text for the input */
  label = input<string>('');

  /** Placeholder text */
  placeholder = input<string>('');

  /** Unique ID for the input element */
  inputId = input<string>(`input-${nextId++}`);

  /** Options array, only used when type is 'select' */
  options = input<{ label: string; value: string }[]>([]);

  /** Internal value of the input */
  value = '';

  /** Whether the input is disabled */
  isDisabled = false;

  /**
   * Called when value changes
   * @param value The changed value
   */
  onChange(value: string): void {}

  /** Called when the input is touched */
  onTouched(): void {}

  /** Change detector reference */
  private cdr = inject(ChangeDetectorRef);

  /**
   * Writes a new value to the element.
   * @param value The new value
   */
  writeValue(value: string): void {
    this.value = value || '';
    this.cdr.markForCheck();
  }

  /**
   * Registers a callback function that is called when the control's value changes in the UI.
   * @param fn The callback function
   */
  registerOnChange(fn: (value: string) => void): void {
    this.onChange = fn;
  }

  /**
   * Registers a callback function that is called by the forms API on initialization to update the form model on blur.
   * @param fn The callback function
   */
  registerOnTouched(fn: () => void): void {
    this.onTouched = fn;
  }

  /**
   * Function that is called by the forms API when the control status changes to or from 'DISABLED'.
   * @param isDisabled Whether the control is disabled
   */
  setDisabledState(isDisabled: boolean): void {
    this.isDisabled = isDisabled;
    this.cdr.markForCheck();
  }

  /**
   * Handles input events for text/textarea
   * @param event The input event
   */
  onInputChange(event: Event): void {
    const val = (event.target as HTMLInputElement | HTMLTextAreaElement).value;
    this.value = val;
    this.onChange(val);
  }

  /**
   * Handles change events for select
   * @param event The select change event
   */
  onSelectChange(event: Event): void {
    const val = (event.target as HTMLSelectElement).value;
    this.value = val;
    this.onChange(val);
  }
}
