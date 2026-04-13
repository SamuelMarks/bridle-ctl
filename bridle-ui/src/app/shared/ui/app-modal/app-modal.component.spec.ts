import { ComponentFixture, TestBed } from '@angular/core/testing';
import { Component } from '@angular/core';
import { AppModalComponent } from './app-modal.component';
import { By } from '@angular/platform-browser';

@Component({
  template: `
    <app-modal
      [isOpen]="isOpen"
      [title]="title"
      [closeOnBackdropClick]="closeOnBackdropClick"
      [showFooter]="showFooter"
      (close)="onClose()"
    >
      <div class="modal-content">Modal Content</div>
      <div modal-footer>
        <button class="footer-btn">OK</button>
      </div>
    </app-modal>
  `,
  imports: [AppModalComponent],
})
class TestHostComponent {
  isOpen = true;
  title = 'Test Modal';
  closeOnBackdropClick = true;
  showFooter = true;
  closed = false;

  onClose() {
    this.closed = true;
  }
}

describe('AppModalComponent', () => {
  let component: TestHostComponent;
  let fixture: ComponentFixture<TestHostComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [TestHostComponent, AppModalComponent],
    }).compileComponents();

    fixture = TestBed.createComponent(TestHostComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should not render when isOpen is false', () => {
    component.isOpen = false;
    fixture.detectChanges();
    const modal = fixture.debugElement.query(By.css('.modal-backdrop'));
    expect(modal).toBeNull();
  });

  it('should render when isOpen is true', () => {
    const modal = fixture.debugElement.query(By.css('.modal-backdrop'));
    expect(modal).toBeTruthy();
  });

  it('should render title and content', () => {
    const title = fixture.debugElement.query(
      By.css('.Box-title'),
    ).nativeElement;
    expect(title.textContent.trim()).toBe('Test Modal');

    const content = fixture.debugElement.query(
      By.css('.modal-content'),
    ).nativeElement;
    expect(content.textContent.trim()).toBe('Modal Content');
  });

  it('should render footer when showFooter is true', () => {
    const footer = fixture.debugElement.query(By.css('.modal-footer'));
    expect(footer).toBeTruthy();

    const btn = footer.query(By.css('.footer-btn')).nativeElement;
    expect(btn.textContent.trim()).toBe('OK');
  });

  it('should not render footer when showFooter is false', () => {
    component.showFooter = false;
    fixture.detectChanges();
    const footer = fixture.debugElement.query(By.css('.modal-footer'));
    expect(footer).toBeNull();
  });

  it('should emit close on close button click', () => {
    const closeBtn = fixture.debugElement.query(
      By.css('.close-button'),
    ).nativeElement;
    closeBtn.click();
    expect(component.closed).toBeTrue();
  });

  it('should emit close on backdrop click if closeOnBackdropClick is true', () => {
    const backdrop = fixture.debugElement.query(
      By.css('.modal-backdrop'),
    ).nativeElement;
    backdrop.click();
    expect(component.closed).toBeTrue();
  });

  it('should not emit close on backdrop click if closeOnBackdropClick is false', () => {
    component.closed = false;
    component.closeOnBackdropClick = false;
    fixture.detectChanges();

    const backdrop = fixture.debugElement.query(
      By.css('.modal-backdrop'),
    ).nativeElement;
    backdrop.click();
    expect(component.closed).toBeFalse();
  });

  it('should not emit close when clicking inside the dialog', () => {
    component.closed = false;
    const dialog = fixture.debugElement.query(
      By.css('.modal-dialog'),
    ).nativeElement;
    dialog.click();
    expect(component.closed).toBeFalse();
  });
});
