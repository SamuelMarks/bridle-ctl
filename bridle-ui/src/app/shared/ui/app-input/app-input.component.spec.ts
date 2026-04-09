import { ComponentFixture, TestBed } from '@angular/core/testing';
import { Component } from '@angular/core';
import { AppInputComponent } from './app-input.component';
import { FormsModule } from '@angular/forms';
import { By } from '@angular/platform-browser';

@Component({
  template: `
    <app-input
      [type]="type"
      [label]="label"
      [placeholder]="placeholder"
      [options]="options"
      [(ngModel)]="val"
      [disabled]="disabled"
    ></app-input>
  `,
  imports: [AppInputComponent, FormsModule]
})
class TestHostComponent {
  type = 'text';
  label = 'Test Label';
  placeholder = 'Enter text';
  options = [{label: 'Opt 1', value: '1'}, {label: 'Opt 2', value: '2'}];
  val = 'initial';
  disabled = false;
}

describe('AppInputComponent', () => {
  let component: TestHostComponent;
  let fixture: ComponentFixture<TestHostComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [TestHostComponent, AppInputComponent, FormsModule]
    }).compileComponents();

    fixture = TestBed.createComponent(TestHostComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
    await fixture.whenStable();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should render a label', () => {
    const label = fixture.debugElement.query(By.css('label')).nativeElement;
    expect(label.textContent).toBe('Test Label');
  });

  it('should render an input by default and bind value', async () => {
    fixture.detectChanges();
    await fixture.whenStable();
    fixture.detectChanges();
    const input = fixture.debugElement.query(By.css('input')).nativeElement;
    expect(input.type).toBe('text');
    expect(input.value).toBe('initial');
    expect(input.placeholder).toBe('Enter text');
  });

  it('should update value on input', () => {
    const input = fixture.debugElement.query(By.css('input')).nativeElement;
    input.value = 'new val';
    input.dispatchEvent(new Event('input'));
    fixture.detectChanges();
    expect(component.val).toBe('new val');
  });

  it('should render a textarea when type is textarea', async () => {
    component.type = 'textarea';
    fixture.detectChanges();
    await fixture.whenStable();
    fixture.detectChanges();
    
    const textarea = fixture.debugElement.query(By.css('textarea')).nativeElement;
    expect(textarea).toBeTruthy();
    expect(textarea.value).toBe('initial');
    
    textarea.value = 'new text';
    textarea.dispatchEvent(new Event('input'));
    expect(component.val).toBe('new text');
  });

  it('should render a select when type is select', async () => {
    component.type = 'select';
    component.val = '1';
    fixture.detectChanges();
    await fixture.whenStable();
    
    const select = fixture.debugElement.query(By.css('select')).nativeElement;
    expect(select).toBeTruthy();
    expect(select.value).toBe('1');
    expect(select.options.length).toBe(2);
    
    select.value = '2';
    select.dispatchEvent(new Event('change'));
    expect(component.val).toBe('2');
  });

  it('should disable input correctly', async () => {
    component.disabled = true;
    fixture.detectChanges();
    await fixture.whenStable();
    fixture.detectChanges();
    
    const input = fixture.debugElement.query(By.css('input')).nativeElement;
    expect(input.disabled).toBeTrue();
  });

  it('should trigger onTouched when blurred', () => {
    const compInstance = fixture.debugElement.query(By.directive(AppInputComponent)).componentInstance;
    let touched = false;
    compInstance.registerOnTouched(() => touched = true);
    
    const input = fixture.debugElement.query(By.css('input')).nativeElement;
    input.dispatchEvent(new Event('blur'));
    
    expect(touched).toBeTrue();
  });
});
